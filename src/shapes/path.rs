use crate::math::cmp::ApproxEq;

use super::{bezier::CubicSlice, point::Point};

#[derive(Debug)]
pub struct Path {
    segments: Box<[PathSegment]>,
    points: Box<[Point]>,
}

impl Path {
    /// Returns an iterator over each subpath in this (possibly) compound path.
    #[must_use]
    pub fn iter(&self) -> SubPathIterator {
        SubPathIterator {
            path: self,
            cursor: 0,
        }
    }
}

/// Iterates over each subpath that makes up a compound path.
pub struct SubPathIterator<'a> {
    path: &'a Path,
    cursor: usize,
}

impl<'a> Iterator for SubPathIterator<'a> {
    type Item = SegmentIterator<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor < self.path.segments.len() {
            let segment = self.path.segments[self.cursor];
            self.cursor += 1;

            Some(SegmentIterator {
                path: self.path,
                cursor: segment.first,
                segment_end: segment.last,
            })
        } else {
            None
        }
    }
}

/// Iterates over each segment in a subpath of a compound path.
pub struct SegmentIterator<'a> {
    path: &'a Path,
    cursor: usize,
    segment_end: usize,
}

impl<'a> Iterator for SegmentIterator<'a> {
    type Item = CubicSlice<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor < self.segment_end {
            let slice = &self.path.points[self.cursor..self.cursor + 4];
            self.cursor += 3;

            Some(CubicSlice::new(slice.try_into().unwrap()))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct PathSegment {
    first: usize,
    last: usize,
}

#[derive(Debug)]
pub enum Error {
    /// The builder cannot complete an action because it requries that a path be
    /// in-progress (with move_to()).
    PathNotStarted,
}

#[derive(Default)]
pub struct Builder {
    segments: Vec<PathSegment>,
    points: Vec<Point>,
    path_start_offset: Option<usize>,
}

// states
//  no curve
//  started curve (has path_start_offset, points.len() > 0)
//  next curve (has path_start_offsset, points.len() > 0)

impl Builder {
    pub fn move_to(&mut self, point: Point) {
        if let Some(path_start_offset) = self.path_start_offset {
            let segment = PathSegment {
                first: path_start_offset,
                last: self.points.len().saturating_sub(1),
            };
            self.segments.push(segment);
        }

        self.path_start_offset = Some(self.points.len());
        self.points.push(point);
    }

    pub fn line_to(&mut self, point: Point) -> Result<(), Error> {
        if self.path_start_offset.is_some() {
            let points = Self::line_points(self.points[self.points.len() - 1], point);
            self.points.push(points[1]);
            self.points.push(points[2]);
            self.points.push(points[2]);
            Ok(())
        } else {
            Err(Error::PathNotStarted)
        }
    }

    pub fn add_cubic(&mut self, p1: Point, p2: Point, p3: Point) -> Result<(), Error> {
        if self.path_start_offset.is_some() {
            self.points.push(p1);
            self.points.push(p2);
            self.points.push(p3);
            Ok(())
        } else {
            Err(Error::PathNotStarted)
        }
    }

    pub fn close(&mut self) -> Result<(), Error> {
        if let Some(path_start_offset) = self.path_start_offset.take() {
            // if the start and end are the same, then we don't need to add a
            // segment else create a straight line between the two and add it to
            // the path.
            let start = self.points[path_start_offset];
            let end = self.points[self.points.len() - 1];

            if !start.approx_eq(end) {
                let points = Self::line_points(
                    self.points[self.points.len() - 1],
                    self.points[path_start_offset],
                );
                self.points.push(points[1]);
                self.points.push(points[2]);
                self.points.push(points[3]);
            }

            self.segments.push(PathSegment {
                first: path_start_offset,
                last: self.points.len() - 1,
            });

            Ok(())
        } else {
            Err(Error::PathNotStarted)
        }
    }

    #[must_use]
    pub fn cursor(&self) -> Option<Point> {
        if self.points.is_empty() {
            None
        } else {
            Some(self.points[self.points.len() - 1])
        }
    }

    #[must_use]
    pub fn build(mut self) -> Path {
        if let Some(path_start_offset) = self.path_start_offset {
            let segment = PathSegment {
                first: path_start_offset,
                last: self.points.len() - 1,
            };
            self.segments.push(segment);
        }

        Path {
            segments: self.segments.into_boxed_slice(),
            points: self.points.into_boxed_slice(),
        }
    }

    fn line_points(p0: Point, p3: Point) -> [Point; 4] {
        let diff = p3 - p0;
        let p1 = diff * 0.25;
        let p2 = diff * 0.75;

        [p0, p1.into(), p2.into(), p3]
    }
}
