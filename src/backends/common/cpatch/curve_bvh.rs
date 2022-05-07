use crate::shapes::{bezier::CubicSlice, path::Path, rect::Rect};

/// Used for graph flattening (intersection) and fill scoring (ray cast).
pub struct CurveBvh<'a> {
    pub nodes: &'a [Node],
    x: &'a [f32],
    y: &'a [f32],
}

impl<'a> CurveBvh<'a> {
    pub fn builder() -> Builder<'a> {
        Builder::default()
    }

    pub fn get(&self, leaf: &Leaf) -> CubicSlice {
        let first_point: usize = leaf.first_point.into();
        CubicSlice::new(
            self.x[first_point..first_point + 4].try_into().unwrap(),
            self.y[first_point..first_point + 4].try_into().unwrap(),
        )
    }

    pub fn get_curve(&self, node: &Node) -> Option<CubicSlice> {
        match node {
            Node::Leaf(leaf) => {
                let first_point: usize = leaf.first_point.into();
                Some(CubicSlice::new(
                    self.x[first_point..first_point + 4].try_into().unwrap(),
                    self.y[first_point..first_point + 4].try_into().unwrap(),
                ))
            }
            Node::Branch { .. } => None,
        }
    }

    pub fn find_intersecting_with(&self, bounds: Rect, buffer: &mut Vec<&Leaf>) {}
}

#[derive(Default)]
pub struct Builder<'a> {
    nodes: Vec<Node>,
    x: &'a [f32],
    y: &'a [f32],
}

impl<'a> Builder<'a> {
    pub fn build(&mut self, path: &Path) -> CurveBvh {
        todo!()
    }
}

pub struct Leaf {
    pub bbox: Rect,
    pub segment_id: u16,
    pub first_point: u16,
}

pub struct Branch {
    pub bbox: Rect,
    pub left_then_right: u16,
}

pub enum Node {
    Leaf(Leaf),
    Branch(Branch),
}

impl Node {
    pub fn leaf(&self) -> Option<&Leaf> {
        match self {
            Node::Leaf(leaf) => Some(leaf),
            _ => None,
        }
    }

    pub fn branch(&self) -> Option<&Branch> {
        match self {
            Node::Branch(branch) => Some(branch),
            _ => None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self {
            Node::Leaf { .. } => true,
            Node::Branch { .. } => false,
        }
    }

    pub fn is_branch(&self) -> bool {
        !self.is_leaf()
    }
}
