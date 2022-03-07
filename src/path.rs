//! 2D paths.

use std::sync::Arc;

use crate::math::Point;

#[derive(Debug, PartialEq)]
enum Command {
    Start,
    LineTo,
    QuadPoint1,
    QuadPoint2,
}

///
pub enum Segment {
    Line { start: Point, end: Point },
    QuadBezier { p0: Point, p1: Point, p2: Point },
}

/// Immutable 2D paths composed of lines and quadratic bezier curves.
#[derive(Clone, Debug, Default)]
pub struct Path {
    // Use `Option` here to avoid alloccating if the path is empty.
    inner: Option<Arc<PathInner>>,
}

impl Path {
    pub fn iter(&self) -> Option<SegmentIter> {
        self.inner.as_ref().map(|s| s.iter())
    }
}

#[derive(Debug, Default)]
struct PathInner {
    commands: Box<[Command]>,
    path_data: Box<[Point]>,
}

impl PathInner {
    fn iter(&self) -> SegmentIter {
        SegmentIter {
            cursor: 0,
            commands: &self.commands,
            path_data: &self.path_data,
            position: Point::default(),
        }
    }
}

/// Iterator over [`Path`] [`Segment`]s.
pub struct SegmentIter<'a> {
    cursor: usize,
    commands: &'a [Command],
    path_data: &'a [Point],
    position: Point,
}

impl<'a> Iterator for SegmentIter<'a> {
    type Item = Segment;

    fn next(&mut self) -> Option<Self::Item> {
        match self.commands.get(self.cursor)? {
            Command::Start => {
                self.position = self.path_data[self.cursor * 2];
                self.next()
            }
            Command::LineTo => {
                let end = self.path_data[self.cursor * 2];

                let s = Segment::Line {
                    start: self.position,
                    end,
                };

                self.position = end;
                self.cursor += 1;
                Some(s)
            }
            Command::QuadPoint1 => {
                let p0 = self.position;
                let p1 = self.path_data[self.cursor * 2];
                self.cursor += 1;

                assert_eq!(self.commands[self.cursor], Command::QuadPoint2);

                let p2 = self.path_data[self.cursor * 2];
                self.cursor += 1;

                self.position = p2;

                Some(Segment::QuadBezier { p0, p1, p2 })
            }
            Command::QuadPoint2 => unreachable!(),
        }
    }
}

/// Constructs a 2D path and produces a [`Path`] object.
#[derive(Default)]
pub struct PathBuilder {
    commands: Vec<Command>,
    path_data: Vec<Point>,
}

impl PathBuilder {
    pub fn start_at(mut self, x: f32, y: f32) -> Self {
        self.commands.push(Command::Start);
        self.path_data.push(Point(x, y));
        self
    }

    pub fn line_to(mut self, x: f32, y: f32) -> Self {
        self.commands.push(Command::LineTo);
        self.path_data.push(Point(x, y));
        self
    }

    pub fn quad_to(mut self, x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        self.commands.push(Command::QuadPoint1);
        self.path_data.push(Point(x1, y1));
        self.commands.push(Command::QuadPoint2);
        self.path_data.push(Point(x2, y2));
        self
    }

    pub fn build(self) -> Path {
        Path {
            inner: self.build_inner().map(|s| Arc::new(s)),
        }
    }

    fn build_inner(mut self) -> Option<PathInner> {
        let first = *self.path_data.first()?;
        {
            let last = *self.path_data.last()?;

            // Close the path
            if first != last {
                self = self.line_to(first.0, first.1);
            }
        }

        Some(PathInner {
            commands: self.commands.into_boxed_slice(),
            path_data: self.path_data.into_boxed_slice(),
        })
    }
}
