//! 2D bounding volume hierarchy.
//!

use crate::{shapes::{rect::{BoundingBox, Rect}, point::Point}, math::vector2::Vec2};

/// A simple bounding volume hierarchy implemented as a binary space partition.
pub struct Bvh<'a, T>
where
    T: BoundingBox,
{
    items: &'a [T],
    nodes: Vec<Node>,
    indirect: Vec<u32>,
}

impl<'a, T> Bvh<'a, T>
where
    T: BoundingBox,
{
    /// Constructs a new bounding volume hierarchy from the given list of
    /// objects.
    pub fn build(items: &'a [T]) -> Self {
        Self::rebuild(
            items,
            Self {
                items,
                nodes: Vec::new(),
                indirect: Vec::new(),
            },
        )
    }

    /// Constructs a new bounding volume hierarchy from the given list of
    /// objects, recycling the memory from an existing bvh.
    pub fn rebuild(items: &'a [T], old: Self) -> Self {
        let mut this = Self {
            items: old.items,
            nodes: old.nodes,
            indirect: old.indirect,
        };

        this.nodes.clear();
        this.nodes.reserve(this.items.len() * 2);

        this.indirect.clear();
        this.indirect.extend(0..this.items.len() as u32);

        bvh_impl::build(&mut this);

        this
    }

    /// Retrieves the list of objects in the bvh.
    pub fn items(&self) -> &[T] {
        self.items
    }

    /// Computes the list of objects that intersect the given rectangle.
    pub fn query_rect_intersection<'t>(&'t self, rect: Rect, out: &mut Vec<&'t T>) {
        bvh_impl::intersect_rect(self, 0, rect, out)
    }

    /// Computes the list of objects that intersect the given ray.
    pub fn query_ray_intersection<'t>(&'t self, _p: Point, _dir: Vec2, _out: &mut Vec<&'t T>) {
        // bvh_impl::intersect_ray(self, 0, p, dir, out)
        todo!()
    }
}

#[derive(Debug)]
struct Node {
    bbox: Rect,
    data: Data,
}

#[derive(Debug)]
enum Data {
    Empty,
    Leaf(Leaf),
    Branch(Branch),
}

#[derive(Clone, Copy, Debug)]
struct Leaf {
    first_indirect: u32,
    count: u32,
}

#[derive(Clone, Copy, Debug)]
struct Branch {
    left_child: u32,
}

mod bvh_impl {
    use super::*;

    pub(super) fn build<T>(bvh: &mut Bvh<T>)
    where
        T: BoundingBox,
    {
        if bvh.items.is_empty() {
            bvh.nodes.push(Node {
                bbox: Rect::default(),
                data: Data::Empty,
            });
            return;
        }

        let aabb = bvh.items.iter().fold(
            Rect::new(f32::MAX, f32::MIN, f32::MAX, f32::MIN),
            |aabb, item| aabb | item.bounding_box(),
        );

        bvh.nodes.push(Node {
            bbox: aabb,
            data: Data::Leaf(Leaf {
                first_indirect: 0,
                count: bvh.items.len() as u32,
            }),
        });

        subdivide(bvh, 0);
    }

    pub(super) fn intersect_rect<'a, T>(
        bvh: &'a Bvh<T>,
        node_idx: usize,
        rect: Rect,
        out: &mut Vec<&'a T>,
    ) where
        T: BoundingBox,
    {
        let node = &bvh.nodes[node_idx];
        match node.data {
            Data::Empty => {}
            Data::Leaf(leaf) => {
                for item in leaf_items_indirect(bvh, node_idx) {
                    if rect.intersects_with(&item.bounding_box()) {
                        out.push(item);
                    }
                }
            }
            Data::Branch(branch) => {
                let left = &bvh.nodes[branch.left_child as usize];
                let right = &bvh.nodes[branch.left_child as usize + 1];

                if left.bbox.intersects_with(&rect) {
                    intersect_rect(bvh, branch.left_child as usize, rect, out);
                }

                if right.bbox.intersects_with(&rect) {
                    intersect_rect(bvh, branch.left_child as usize + 1, rect, out);
                }
            }
        }
    }

    enum SplitAxis {
        X,
        Y,
    }

    struct Split {
        axis: SplitAxis,
        position: f32,
        cost: f32,
    }

    struct IterIndirect<'a, T> {
        items: &'a [T],
        indirect: &'a [u32],
        index: usize,
    }

    impl<'a, T> Iterator for IterIndirect<'a, T>
    where
        T: BoundingBox,
    {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.indirect.len() {
                let item = &self.items[self.indirect[self.index] as usize];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    fn compute_bounds_indirect<T>(items: &[T], indirect: &[u32]) -> Rect
    where
        T: BoundingBox,
    {
        let mut aabb = items[indirect[0] as usize].bounding_box();
        for indirect in &indirect[1..] {
            aabb |= items[*indirect as usize].bounding_box();
        }
        aabb
    }

    fn leaf_items_indirect<'a, T>(bvh: &'a Bvh<T>, node_idx: usize) -> IterIndirect<'a, T>
    where
        T: BoundingBox,
    {
        let node = &bvh.nodes[node_idx];
        match &node.data {
            Data::Leaf(leaf) => IterIndirect {
                items: bvh.items,
                indirect: &bvh.indirect
                    [leaf.first_indirect as usize..(leaf.first_indirect + leaf.count) as usize],
                index: 0,
            },
            _ => panic!("expected leaf node"),
        }
    }

    pub fn subdivide<T>(bvh: &mut Bvh<T>, node_idx: usize)
    where
        T: BoundingBox,
    {
        let node = &mut bvh.nodes[node_idx];
        let leaf = match node.data {
            Data::Leaf(leaf) => leaf,
            _ => panic!("expected leaf node"),
        };
        let node_cost = leaf.count as f32 * node.bbox.area();

        let best_split = find_split_axis(bvh, node_idx);

        if node_cost < best_split.cost {
            return;
        }

        let mut i = leaf.first_indirect as isize;
        let mut j = i + leaf.count as isize - 1;

        while i <= j {
            let centroid = bvh.items[bvh.indirect[i as usize] as usize]
                .bounding_box()
                .centroid();

            let value = match best_split.axis {
                SplitAxis::X => centroid.x,
                SplitAxis::Y => centroid.y,
            };

            if value < best_split.position {
                i += 1;
            } else {
                bvh.indirect.swap(i as usize, j as usize);
                j -= 1;
            }
        }

        let left_count = i as usize - leaf.first_indirect as usize;
        if left_count == 0 || left_count == leaf.count as usize {
            return;
        }

        let indirect = &mut bvh.indirect
            [leaf.first_indirect as usize..(leaf.first_indirect + leaf.count) as usize];

        let left_child_idx = bvh.nodes.len();
        bvh.nodes.push(Node {
            bbox: compute_bounds_indirect(bvh.items, &indirect[..left_count]),
            data: Data::Leaf(Leaf {
                first_indirect: leaf.first_indirect,
                count: left_count as u32,
            }),
        });

        let right_child_idx = bvh.nodes.len();
        bvh.nodes.push(Node {
            bbox: compute_bounds_indirect(bvh.items, &indirect[left_count..]),
            data: Data::Leaf(Leaf {
                first_indirect: i as u32,
                count: leaf.count - left_count as u32,
            }),
        });

        bvh.nodes[node_idx].data = Data::Branch(Branch {
            left_child: left_child_idx as u32,
        });

        subdivide(bvh, left_child_idx);
        subdivide(bvh, right_child_idx);
    }

    fn find_split_axis<T>(bvh: &Bvh<T>, node_idx: usize) -> Split
    where
        T: BoundingBox,
    {
        let mut best_pos = 0.0;
        let mut best_axis = SplitAxis::X;
        let mut best_cost = f32::MAX;

        for item in leaf_items_indirect(bvh, node_idx) {
            let candidate_pos = item.bounding_box().centroid();

            let cost_x = evalate_sah(bvh, node_idx, SplitAxis::X, candidate_pos.x);
            if cost_x < best_cost {
                best_cost = cost_x;
                best_axis = SplitAxis::X;
                best_pos = candidate_pos.x;
            }

            let cost_y = evalate_sah(bvh, node_idx, SplitAxis::Y, candidate_pos.y);
            if cost_y < best_cost {
                best_cost = cost_y;
                best_axis = SplitAxis::Y;
                best_pos = candidate_pos.y;
            }
        }

        Split {
            axis: best_axis,
            position: best_pos,
            cost: best_cost,
        }
    }

    fn evalate_sah<T>(bvh: &Bvh<T>, node_idx: usize, axis: SplitAxis, pos: f32) -> f32
    where
        T: BoundingBox,
    {
        let mut left = None;
        let mut right = None;
        let mut num_left = 0;
        let mut num_right = 0;

        for item in leaf_items_indirect(bvh, node_idx) {
            let aabb = item.bounding_box();
            let centroid = aabb.centroid();

            let value = match axis {
                SplitAxis::X => centroid.x,
                SplitAxis::Y => centroid.y,
            };

            if value < pos {
                num_left += 1;
                left = left.map_or(Some(aabb), |b: Rect| Some(b | aabb));
            } else {
                num_right += 1;
                right = right.map_or(Some(aabb), |b: Rect| Some(b | aabb));
            }
        }

        let left_cost = num_left as f32 * left.map_or(0.0, |b| b.area());
        let right_cost = num_right as f32 * right.map_or(0.0, |b| b.area());
        let cost = left_cost + right_cost;

        if cost > 0.0 {
            cost
        } else {
            f32::MAX
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bvh_node_size() {
        assert_eq!(std::mem::size_of::<Node>(), 28);
    }

    #[test]
    fn empty_bvh() {
        let bvh = Bvh::<Rect>::build(&[]);
        assert_eq!(bvh.items().len(), 0);
        let mut out = Vec::new();
        bvh.query_rect_intersection(Rect::new(1.0, 2.0, 1.0, 2.0), &mut out);
        assert!(out.is_empty());
    }

    #[test]
    fn five_rects_bvh() {
        //
        // *-------*                  *-------*
        // |       |                  |       |
        // |   0   |                  |   1   |
        // |     .-+-------.          |       |
        // *-----|-*       |          *------*
        //       |    4    |
        // *-----+-*     *-+-----*
        // |     *-+-----+-*     |
        // |   2   |     |   3   |
        // |       |     |       |
        // *-------*     *-------*

        let rects = [
            Rect::new(0.0, 1.0, 0.0, 1.0),
            Rect::new(4.0, 5.0, 0.0, 1.0),
            Rect::new(0.0, 1.0, 2.0, 3.0),
            Rect::new(2.0, 3.0, 2.0, 3.0),
            Rect::new(0.5, 2.5, 0.5, 2.5),
        ];

        assert!(!rects[0].intersects_with(&rects[1]));
        assert!(!rects[0].intersects_with(&rects[2]));
        assert!(!rects[0].intersects_with(&rects[3]));
        assert!(!rects[1].intersects_with(&rects[2]));
        assert!(!rects[1].intersects_with(&rects[3]));
        assert!(!rects[2].intersects_with(&rects[3]));

        assert!(rects[0].intersects_with(&rects[4]));
        assert!(!rects[1].intersects_with(&rects[4]));
        assert!(rects[2].intersects_with(&rects[4]));
        assert!(rects[3].intersects_with(&rects[4]));

        let bvh = Bvh::<Rect>::build(&rects);
        assert_eq!(bvh.nodes.len(), rects.len() * 2 - 1);

        {
            let mut out = Vec::new();
            bvh.query_rect_intersection(rects[0], &mut out);
            assert!(out.len() == 2);
        }

        {
            let mut out = Vec::new();
            bvh.query_rect_intersection(rects[1], &mut out);
            assert!(out.len() == 1);
        }

        {
            let mut out = Vec::new();
            bvh.query_rect_intersection(rects[2], &mut out);
            assert!(out.len() == 2);
        }

        {
            let mut out = Vec::new();
            bvh.query_rect_intersection(rects[3], &mut out);
            assert!(out.len() == 2);
        }

        {
            let mut out = Vec::new();
            bvh.query_rect_intersection(rects[4], &mut out);
            assert!(out.len() == 4);
        }

        {
            let mut out = Vec::new();
            bvh.query_rect_intersection(Rect::new(-1.0, -0.5, 1.0, 2.0), &mut out);
            assert!(out.is_empty());
        }
    }
}
