//! Code for translating bezier paths into CPatches, as described in
//! "Hierarchical Rasterization of Curved Primitives for Vector Graphics
//! Rendering on the GPU" by Dockter et al., 2019.
//!
//! The process for this conversion is quite involved, and is broken down to the
//! following steps:
//! 1. Graph flattening: Every curve within the path is tested for intersection
//!    with every other curve. Intersecting curves are replaced with subdivided
//!    counterparts, where the curves meet at the intersection.
//!     1a. Normalization: The curves are normalized to the unit square.
//! 2. Fill scoring: A ray is shot out from each curve at its midpoint towards
//!    the edge of the path. The fill score is then computed according to the
//!    desired fill rule "Non-Zero" or "Even-Odd". This is accelerated by the
//!    use of a bounding volume hierarchy.
//! 3. Cycle extraction: TBD
//! 4. Patch cuttiing: TBD
//! 5. Self-intersection cutting: TBD
//! 6. Extension correction: TBD

use crate::{
    math::cmp::{max, min},
    shapes::{bezier::Bezier, path::Path, rect::Rect},
};

mod change_list;
mod curve_bvh;

pub use change_list::ChangeList;
pub use curve_bvh::CurveBvh;

pub fn normalize(path: &mut Path) -> Rect {
    let rect = {
        let mut min_x = path.x[0];
        let mut min_y = path.y[0];
        let mut max_x = path.x[0];
        let mut max_y = path.y[0];

        for x in &path.x[1..] {
            min_x = min!(min_x, *x);
            max_x = max!(max_x, *x);
        }

        for y in &path.y[1..] {
            min_y = min!(min_y, *y);
            max_y = max!(max_y, *y);
        }

        Rect::new(min_x, max_x, min_y, max_y)
    };

    let offset_x = rect.left;
    let offset_y = rect.top;
    let div_x = rect.width();
    let div_y = rect.height();

    for x in &mut path.x {
        *x = (*x - offset_x) / div_x;
    }

    for y in &mut path.y {
        *y = (*y - offset_y) / div_y;
    }

    rect
}

pub fn flatten<'a>(
    path: &mut Path,
    change_buffer: &mut change_list::ChangeList,
    bvh_builder: &'a mut curve_bvh::Builder,
) -> curve_bvh::CurveBvh<'a> {
    // CONCERN (straivers): Normalization may accidentally produce denormal numbers for
    // very large paths, which have an outsized impact on performance. A
    // possible solution would be to use f64 instead, though it would involve
    // quite a bit of work. Of course, the big issue here is that the longer we
    // wait to make the transition, the harder it will become.

    // normalize?

    change_buffer.clear();
    let mut intersection_buffer = Vec::new();

    let mut candidates = 0;
    let mut intersections = 0;

    let mut iteration = 0;
    loop {
        let mut found_intersection = false;
        let bvh = bvh_builder.build(path);

        for (node, node_idx) in bvh.nodes.iter().zip(0..).filter(|node| node.0.is_leaf()) {
            if iteration > 0 && node.last_touched.get() == iteration {
                continue;
            }

            let leaf = node.leaf().unwrap();
            bvh.find_intersecting_with(node_idx, &mut intersection_buffer);

            candidates += intersection_buffer.len();

            // println!("node");

            for (curve, segment_id, first_point) in bvh.curves_in(leaf, path) {
                // println!("curve");

                for (c_node_id, (c_curve, c_segment_id, c_first_point)) in intersection_buffer
                    .iter()
                    .cloned()
                    .filter(|(c_node_id, _)| {
                        bvh.nodes[*c_node_id as usize].last_touched.get() != iteration
                    })
                    .flat_map(|(i, leaf)| std::iter::repeat(i).zip(bvh.curves_in(leaf, path)))
                {
                    println!("\t curve {}, candidate {}", node_idx, c_node_id);
                    let (node_splits, candidate_splits) = curve.find_intersections(&c_curve);

                    if !node_splits.is_empty() {
                        intersections += node_splits.len();

                        change_buffer.replace(segment_id, first_point, |x, y| {
                            curve.splitn(&node_splits, x, y);
                        });

                        change_buffer.replace(c_segment_id, c_first_point, |x, y| {
                            c_curve.splitn(&candidate_splits, x, y);
                        });

                        node.last_touched.replace(iteration);
                        bvh.nodes[c_node_id as usize]
                            .last_touched
                            .replace(iteration);

                        // Skip the rest of the intersection buffer since we've
                        // already made changes to the bvh that won't be reflected
                        // until it's been rebuilt.
                        found_intersection = true;
                        break;
                    }
                }
            }
        }

        iteration += 1;

        if found_intersection {
            change_buffer.apply(path);
        } else {
            break;
        }
    }

    println!("{} candidates, {} intersections", candidates, intersections);

    // TODO (straivers): This is a hack to get around fixed-lifetime issues.
    // Ideally, we would return the bvh if !found_intersection, but that's not
    // possible because it would require the bvh to have a lifetime of 'a,
    // preventing us from recreating the bvh every iteration (multiple mutable
    // borrows). A possible option would be to somehow 'rebuild' the bvh without
    // actually doing any of the work, but I am not sure how to do that safely.
    //
    // A token produced by `CurveBvh` that can be used to recover it would be
    // ideal, 'cept that it is entirely unsafe. Now, if we only use it here, it
    // should be fine, but I remain uncomfortable with the idea if it can be
    // avoided.
    bvh_builder.build(path)
}

pub fn compute_fill_scores(path: &Path, bvh: curve_bvh::CurveBvh, score_buffer: &mut Vec<u16>) {
    todo!()
}

pub fn extract_cycles(path: Path) -> Vec<Path> {
    todo!()
}

/// Combines patch cutting, self-intersection cutting, and extension correction.
pub fn finalize(path: &mut Path) {}
