//! Code for translating bezier paths into CPatches, as described in
//! "Hierarchical Rasterization of Curved Primitives for Vector Graphics
//! Rendering on the GPU" by Dockter et al., 2019.
//!
//! The process for this conversion is quite involved, and is broken down to the
//! following steps:
//! 1. Path normalization: The points within the path are brought to the range
//!    0..1, with scale and offset preserved with a corresponding `Rect`
//!    instance.
//! 2. Graph flattening: Every curve within the path is tested for intersection
//!    with every other curve. Intersecting curves are replaced with subdivided
//!    counterparts, where the curves meet at the intersection.
//! 3. Fill scoring: A ray is shot out from each curve at its midpoint towards
//!    the edge of the path. The fill score is then computed according to the
//!    desired fill rule "Non-Zero" or "Even-Odd". This is accelerated by the
//!    use of a bounding volume hierarchy.
//! 4. Cycle extraction: TBD
//! 5. Patch cuttiing: TBD
//! 6. Self-intersection cutting: TBD
//! 7. Extension correction: TBD

use crate::shapes::path::Path;

mod change_list;
mod curve_bvh;

pub fn normalize(path: &mut Path) {
    todo!()
}

pub fn flatten<'a, 'b>(
    path: &'a mut Path,
    change_buffer: &mut change_list::ChangeList,
    bvh_builder: &'b mut curve_bvh::Builder,
) -> curve_bvh::CurveBvh<'b>
where
    'b: 'a,
{
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
