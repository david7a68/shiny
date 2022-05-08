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

use crate::shapes::{bezier::Bezier, path::Path};

use self::change_list::ChangeList;

mod change_list;
mod curve_bvh;

pub fn flatten<'a, 'b>(
    path: &'a mut Path,
    change_buffer: &mut change_list::ChangeList,
    bvh_builder: &'b mut curve_bvh::Builder,
) -> curve_bvh::CurveBvh<'b>
where
    'b: 'a,
{
    // CONCERN (straivers): Normalization may accidentally produce denormalized numbers for
    // very large paths, which have an outsized impact on performance. A
    // possible solution would be to use f64 instead, though it would involve
    // quite a bit of work. Of course, the big issue here is that the longer we
    // wait to make the transition, the harder it will become.

    change_buffer.clear();
    let mut bvh = bvh_builder.build(path);
    let mut intersection_buffer = Vec::new();

    let mut found_intersection = true;
    while found_intersection {
        found_intersection = false;
        for node in bvh.nodes.iter().filter(|node| node.is_leaf()) {
            let leaf = node.leaf().unwrap();
            bvh.find_intersecting_with(leaf.bbox, &mut intersection_buffer);

            for candidate in intersection_buffer.iter() {
                let curve = bvh.get(leaf);
                let candidate_curve = bvh.get(candidate);
                let (a_intersections, b_intersections) = curve.find_intersections(&candidate_curve);

                // PROBLEM: Bezier::find_intersections fails in the case of two
                // curves with colinear segments, reporting intersections that are
                // very close together.
                //
                // However, we are ignoring that fact for the moment.

                if a_intersections.is_empty() {
                    // Continue looking for the next intersection.
                    continue;
                }

                change_buffer.replace(leaf.segment_id, leaf.first_point, |x, y| {
                    curve.splitn(&a_intersections, x, y);
                });
                change_buffer.replace(candidate.segment_id, candidate.first_point, |x, y| {
                    candidate_curve.splitn(&b_intersections, x, y);
                });

                // Skip the rest of the intersection buffer since we've already
                // made changes to the bvh that won't be reflected until it's
                // been rebuilt.
                found_intersection = true;
                break;
            }
        }

        change_buffer.apply(path);
        bvh = bvh_builder.build(path);
    }

    todo!()
}

pub fn compute_fill_scores(path: &Path, bvh: curve_bvh::CurveBvh, score_buffer: &mut Vec<u16>) {
    todo!()
}

pub fn extract_cycles(path: Path) -> Vec<Path> {
    todo!()
}

/// Combines patch cutting, self-intersection cutting, and extension correction.
pub fn finalize(path: &mut Path) {}
