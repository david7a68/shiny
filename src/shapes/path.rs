use crate::math::cmp::ApproxEq;

use super::{bezier::CubicSlice, point::Point, rect::Rect};

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
                segment_end: segment.one_past_end,
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
        if self.cursor + 3 < self.segment_end {
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
    one_past_end: usize,
    bounds: Rect,
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
    segment_index: Option<usize>,
}

// states
//  no curve
//  started curve (has path_start_offset, points.len() > 0)
//  next curve (has path_start_offsset, points.len() > 0)

impl Builder {
    pub fn move_to(&mut self, point: Point) {
        if let Some(index) = self.segment_index.take() {
            // Close out the previous segment.
            self.segments[index].one_past_end = self.points.len();
        }

        self.segment_index = Some(self.segments.len());
        self.segments.push(PathSegment {
            first: self.points.len(),
            one_past_end: 0,
            bounds: Rect::new(point.x, point.x, point.y, point.y),
        });

        self.points.push(point);
    }

    pub fn line_to(&mut self, point: Point) -> Result<(), Error> {
        if let Some(index) = self.segment_index {
            // Adjust bounding box to include this line.
            self.segments[index].bounds += Rect::enclosing(&[point]);

            // Add the line as a cubic bezier.
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
        if let Some(index) = self.segment_index {
            self.segments[index].bounds += Rect::enclosing(&[p1, p2, p3]);

            self.points.push(p1);
            self.points.push(p2);
            self.points.push(p3);
            Ok(())
        } else {
            Err(Error::PathNotStarted)
        }
    }

    pub fn close(&mut self) -> Result<(), Error> {
        if let Some(index) = self.segment_index.take() {
            // Create a line closing the path iff the start and end points are
            // not equal.
            let start = self.points[self.segments[index].first];
            let end = self.points[self.points.len() - 1];

            if !start.approx_eq(end) {
                let points = Self::line_points(end, start);
                self.points.push(points[1]);
                self.points.push(points[2]);
                self.points.push(points[3]);
            }

            // No need to adjust the bounding box, since we're working within
            // the convex hull of the bounding box (no way to generate a line
            // that lies outside of the bounds).

            self.segments[index].one_past_end = self.points.len();

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

    pub fn build(mut self) -> Result<Path, Error> {
        // Swallow the error, because the path may have already been closed by
        // the user.
        let _ = self.close();

        Ok(Path {
            segments: self.segments.into_boxed_slice(),
            points: self.points.into_boxed_slice(),
        })
    }

    fn line_points(p0: Point, p3: Point) -> [Point; 4] {
        let diff = p3 - p0;
        let p1 = diff * 0.25;
        let p2 = diff * 0.75;

        [p0, p1.into(), p2.into(), p3]
    }
}
