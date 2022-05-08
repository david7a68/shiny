use std::cell::Cell;

use crate::shapes::{
    bezier::{Bezier, CubicSlice},
    path::Path,
    point::Point,
    rect::Rect,
};

/// Used for graph flattening (intersection) and fill scoring (ray cast).
pub struct CurveBvh<'a> {
    pub nodes: &'a mut [Node],
    pub curves: &'a [Curve],
}

impl<'a> CurveBvh<'a> {
    pub fn storage() -> Builder {
        Builder::default()
    }

    pub fn curves_in<'p>(
        &self,
        leaf: Leaf,
        path: &'p Path,
    ) -> impl Iterator<Item = (CubicSlice<'p>, u16, u16)> + 'a
    where
        'p: 'a,
    {
        self.curves[leaf.first_curve as usize..(leaf.first_curve + leaf.num_curves) as usize]
            .iter()
            .map(|curve| {
                (
                    CubicSlice {
                        x: path.x[curve.first_point as usize..curve.first_point as usize + 4]
                            .try_into()
                            .unwrap(),
                        y: path.y[curve.first_point as usize..curve.first_point as usize + 4]
                            .try_into()
                            .unwrap(),
                    },
                    curve.segment_id,
                    curve.first_point,
                )
            })
    }

    pub fn find_intersecting_with(&self, node_idx: u32, buffer: &mut Vec<(u32, Leaf)>) {
        buffer.clear();
        self.intersect(node_idx, self.nodes[node_idx as usize].bbox, buffer, 0);
    }

    pub fn intersect(
        &self,
        origin_idx: u32,
        bounds: Rect,
        buffer: &mut Vec<(u32, Leaf)>,
        current_node_idx: u32,
    ) {
        if origin_idx != current_node_idx {
            let node = &self.nodes[current_node_idx as usize];
            match node.data {
                Data::Empty => unreachable!(),
                Data::Leaf(leaf) => {
                    buffer.push((current_node_idx, leaf));
                }
                Data::Branch(branch) => {
                    self.intersect(origin_idx, bounds, buffer, branch.left_then_right);
                    self.intersect(origin_idx, bounds, buffer, branch.left_then_right + 1);
                }
            }
        }
    }
}

#[derive(Default)]
pub struct Builder {
    nodes: Vec<Node>,
    curves: Vec<Curve>,
}

impl Builder {
    pub fn build(&mut self, path: &Path) -> CurveBvh {
        let num_curves = {
            let mut count = 0;
            for segment in &path.segments {
                debug_assert_eq!((segment.length - 1) % 3, 0);
                count += (segment.length - 1) / 3;
            }
            count
        } as usize;

        self.nodes.clear();
        self.nodes.reserve(2 * num_curves);

        self.curves.clear();
        self.curves.reserve(num_curves);

        let mut offset = 0;
        for (segment, segment_id) in path.iter().zip(0..) {
            let it = segment.zip((0..).step_by(3)).map(|(c, i)| Curve {
                segment_id,
                centroid: c.coarse_bounds().centroid(),
                first_point: offset + i,
            });

            self.curves.extend(it);
            offset += path.segments[segment_id as usize].length;
        }

        debug_assert_eq!(self.curves.capacity(), self.curves.len());

        let root = Node {
            bbox: Self::compute_bounds(&self.curves, path),
            last_touched: Cell::new(0),
            data: Data::Leaf(Leaf {
                first_curve: 0,
                num_curves: num_curves as u16,
            }),
        };

        self.nodes.push(root);
        self.nodes.push(Node {
            bbox: Rect::default(),
            last_touched: Cell::new(0),
            data: Data::Empty,
        });

        self.subdivide(0, path);

        CurveBvh {
            nodes: &mut self.nodes,
            curves: &self.curves,
        }
    }

    fn get_slice(curve: Curve, path: &Path) -> CubicSlice {
        let first_point: usize = curve.first_point.into();
        CubicSlice::new(
            path.x[first_point..first_point + 4].try_into().unwrap(),
            path.y[first_point..first_point + 4].try_into().unwrap(),
        )
    }

    fn compute_bounds(curves: &[Curve], path: &Path) -> Rect {
        if curves.is_empty() {
            Rect::default()
        } else {
            curves[1..].iter().fold(
                Self::get_slice(curves[0], path).coarse_bounds(),
                |bounds, curve| bounds | Self::get_slice(*curve, path).coarse_bounds(),
            )
        }
    }

    fn subdivide(&mut self, node_id: u32, path: &Path) {
        // Need to get left_idx here because we borrow self.nodes mutably and
        // keep it until we split the node.
        let left_idx = self.nodes.len() as u32;

        let (split_axis, split_pos, split_cost) = self.find_split_axis(node_id, path);
        let node = &mut self.nodes[node_id as usize];
        let leaf = node.leaf().unwrap();

        if leaf.num_curves <= 1 {
            return;
        }

        let mut i = leaf.first_curve as isize;
        let mut j = i + leaf.num_curves as isize - 1;
        match split_axis {
            SplitAxis::X => {
                while i <= j {
                    if self.curves[i as usize].centroid.x < split_pos {
                        i += 1;
                    } else {
                        self.curves.swap(i as usize, j as usize);
                        j -= 1;
                    }
                }
            }
            SplitAxis::Y => {
                while i <= j {
                    if self.curves[i as usize].centroid.y < split_pos {
                        i += 1;
                    } else {
                        self.curves.swap(i as usize, j as usize);
                        j -= 1;
                    }
                }
            }
        }

        let left_count = u16::try_from(i).unwrap() - leaf.first_curve;
        let node_cost = leaf.num_curves as f32 * node.bbox.area();

        println!(
            "num curves: {}, bbox area: {}",
            leaf.num_curves,
            node.bbox.area()
        );
        println!("node cost: {}, split cost: {}", node_cost, split_cost);

        if split_cost >= node_cost {
            return;
        }

        node.data = Data::Branch(Branch {
            left_then_right: left_idx,
        });

        self.nodes.push(Node {
            bbox: Self::compute_bounds(&self.curves[leaf.first_curve as usize..i as usize], path),
            last_touched: Cell::new(0),
            data: Data::Leaf(Leaf {
                first_curve: leaf.first_curve,
                num_curves: left_count,
            }),
        });

        self.nodes.push(Node {
            bbox: Self::compute_bounds(
                &self.curves[i as usize..(leaf.first_curve + leaf.num_curves) as usize],
                path,
            ),
            last_touched: Cell::new(0),
            data: Data::Leaf(Leaf {
                first_curve: i as u16,
                num_curves: leaf.num_curves - left_count as u16,
            }),
        });
    }

    fn find_split_axis(&self, node_id: u32, path: &Path) -> (SplitAxis, f32, f32) {
        let node = &self.nodes[node_id as usize];

        let mut best_axis = SplitAxis::X;
        let mut best_pos = 0.0;
        let mut best_cost = f32::MAX;

        for i in 0..node.leaf().unwrap().num_curves {
            let curve = &self.curves[(node.leaf().unwrap().first_curve + i) as usize];
            let candidate_pos = curve.centroid.x;
            let cost = self.evaluate_sah(node, SplitAxis::X, candidate_pos, path);
            if cost < best_cost {
                best_axis = SplitAxis::X;
                best_pos = candidate_pos;
                best_cost = cost;
            }
        }

        for i in 0..node.leaf().unwrap().num_curves {
            let curve = &self.curves[(node.leaf().unwrap().first_curve + i) as usize];
            let candidate_pos = curve.centroid.y;
            let cost = self.evaluate_sah(node, SplitAxis::Y, candidate_pos, path);
            if cost < best_cost {
                best_axis = SplitAxis::Y;
                best_pos = candidate_pos;
                best_cost = cost;
            }
        }

        (best_axis, best_pos, best_cost)
    }

    fn evaluate_sah(&self, node: &Node, axis: SplitAxis, position: f32, path: &Path) -> f32 {
        let mut left = None;
        let mut num_left = 0;

        let mut right = None;
        let mut num_right = 0;

        for i in 0..node.leaf().unwrap().num_curves {
            let curve = &self.curves[(node.leaf().unwrap().first_curve + i) as usize];
            let slice = Self::get_slice(*curve, path);

            let curve_pos = match axis {
                SplitAxis::X => curve.centroid.x,
                SplitAxis::Y => curve.centroid.y,
            };

            if curve_pos < position {
                num_left += 1;
                left = left.map(|b: Rect| b | slice.coarse_bounds());
            } else {
                num_right += 1;
                right = right.map(|b: Rect| b | slice.coarse_bounds());
            }
        }

        // Return without testing for <= 0 as described in article...
        left.map_or(0.0, |lb| num_left as f32 * lb.area())
            + right.map_or(0.0, |rb| num_right as f32 * rb.area())
    }
}

#[derive(Clone, Copy)]
pub struct Curve {
    pub centroid: Point,
    pub segment_id: u16,
    pub first_point: u16,
}

#[derive(Clone, Copy, Debug)]
pub struct Leaf {
    /// The index of the first curve in this leaf.
    pub first_curve: u16,
    /// The number of curves in this leaf.
    pub num_curves: u16,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Branch {
    /// The index of the left child. The right child is at index + 1. This is a
    /// u32 instead of a u16 because we have 2N nodes for N curves.
    pub left_then_right: u32,
}

pub enum SplitAxis {
    X,
    Y,
}

#[derive(Debug)]
pub struct Node {
    pub bbox: Rect,
    pub last_touched: Cell<u32>,
    pub data: Data,
}

#[derive(Debug)]
pub enum Data {
    Empty,
    Leaf(Leaf),
    Branch(Branch),
}

impl Node {
    pub fn leaf(&self) -> Option<Leaf> {
        match self.data {
            Data::Leaf(leaf) => Some(leaf),
            _ => None,
        }
    }

    pub fn branch(&self) -> Option<Branch> {
        match self.data {
            Data::Branch(branch) => Some(branch),
            _ => None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self.data {
            Data::Leaf { .. } => true,
            Data::Empty => false,
            Data::Branch { .. } => false,
        }
    }

    pub fn is_branch(&self) -> bool {
        !self.is_leaf()
    }
}

#[test]
fn f() {
    println!("{}", std::mem::size_of::<Node>());
}
