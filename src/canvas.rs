//! Render target.

use std::vec;

use crate::{
    color::Color,
    math::Float2,
    path::{Path, PathBuilder},
};

/// Information for painting paths such as color.
#[derive(Clone, Copy, Debug, Default)]
pub struct Paint {
    color: Color,
}

/// Drawing Context.
pub struct Canvas {
    draws: Vec<Node>,
    paths: Vec<Path>,
    paints: Vec<Paint>,
    parents: Vec<usize>,
}

impl Canvas {
    pub fn new(width: f32, height: f32, background_color: Color) -> Self {
        let clip = PathBuilder::default()
            .line_to(0.0, height)
            .line_to(width, height)
            .line_to(width, 0.0)
            .build();

        let background = Paint {
            color: background_color,
        };

        let mut this = Self {
            draws: vec![],
            paths: vec![],
            paints: vec![],
            parents: vec![],
        };

        {
            // Set up clip boundary
            this.paths.push(clip.clone());
            this.draws.push(Node {
                kind: Kind::Clip,
                offset: Float2::default(),
                path_index: 0,
                children: vec![],
                paint_index: 0,
            });
            this.parents.push(0);
        }
        this.fill_path(clip, Float2::default(), background);
        this
    }

    pub fn push_clip(&mut self, path: Path, offset: Float2) {
        let path_i = self.paths.len();
        self.paths.push(path);

        let index_i = self.draws.len();
        self.draws.push(Node {
            kind: Kind::Clip,
            offset,
            path_index: path_i as u32,
            children: vec![],
            paint_index: 0,
        });

        let parent = &mut self.draws[*self.parents.last().unwrap()];
        assert_eq!(parent.kind, Kind::Clip);
        parent.children.push(index_i);
        self.parents.push(index_i);
    }

    pub fn pop_clip(&mut self) {
        self.parents.pop();
    }

    pub fn fill_path(&mut self, path: Path, offset: Float2, paint: Paint) {
        let path_i = self.paths.len();
        self.paths.push(path);

        let paint_i = self.paints.len();
        self.paints.push(paint);

        let index_i = self.draws.len();
        self.draws.push(Node {
            kind: Kind::Fill,
            offset,
            path_index: path_i as u32,
            children: vec![],
            paint_index: paint_i as u32,
        });

        let parent = &mut self.draws[*self.parents.last().unwrap()];
        assert_eq!(parent.kind, Kind::Clip);
        parent.children.push(index_i);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Kind {
    Clip,
    Fill,
}

#[derive(Debug)]
struct Node {
    kind: Kind,
    offset: Float2,
    path_index: u32,

    // Only used by Kind::Clip
    children: Vec<usize>,

    // Only used by Kind::Fill
    paint_index: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_init() {
        let canvas = Canvas::new(
            100.0,
            100.0,
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            },
        );

        assert_eq!(canvas.draws.len(), 2);
        assert_eq!(canvas.paths.len(), 2);
        assert_eq!(canvas.paints.len(), 1);
        assert_eq!(canvas.parents.len(), 1);
    }
}
