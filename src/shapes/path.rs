use crate::math::cmp::ApproxEq;

use super::{bezier::CubicSlice, point::Point};

#[derive(Debug)]
pub struct Path {
    pub segments: Vec<Segment>,
}

/// Iterates over each segment in a subpath of a compound path.
pub struct SegmentIterator<'a> {
    path: &'a [Point],
    cursor: usize,
}

impl<'a> Iterator for SegmentIterator<'a> {
    type Item = CubicSlice<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor + 3 < self.path.len() {
            let slice = &self.path[self.cursor..self.cursor + 4];
            self.cursor += 3;

            Some(CubicSlice::new(slice.try_into().unwrap()))
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct Segment {
    points: Vec<Point>,
}

impl Segment {
    pub fn curves(&self) -> SegmentIterator {
        SegmentIterator {
            path: &self.points,
            cursor: 0,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    /// The builder cannot complete an action because it requries that a path be
    /// in-progress (with move_to()).
    PathNotStarted,
}

#[derive(Default)]
pub struct Builder {
    segments: Vec<Segment>,
}

impl Builder {
    pub fn move_to(&mut self, point: Point) {
        self.segments.push(Segment {
            points: vec![point],
        });
    }

    pub fn line_to(&mut self, point: Point) -> Result<(), Error> {
        if let Some(segment) = self.segments.last_mut() {
            let points = Self::line_as_cubic(segment.points[segment.points.len() - 1], point);
            segment.points.push(points[1]);
            segment.points.push(points[2]);
            segment.points.push(points[2]);
            Ok(())
        } else {
            Err(Error::PathNotStarted)
        }
    }

    pub fn add_cubic(&mut self, p1: Point, p2: Point, p3: Point) -> Result<(), Error> {
        if let Some(segment) = self.segments.last_mut() {
            segment.points.push(p1);
            segment.points.push(p2);
            segment.points.push(p3);
            Ok(())
        } else {
            Err(Error::PathNotStarted)
        }
    }

    pub fn close(&mut self) -> Result<(), Error> {
        if let Some(segment) = self.segments.last_mut() {
            // Create a line closing the path iff the start and end points are
            // not equal.
            let start = segment.points.first().unwrap();
            let end = segment.points[segment.points.len() - 1];

            if !start.approx_eq(&end) {
                let points = Self::line_as_cubic(end, *start);
                segment.points.push(points[1]);
                segment.points.push(points[2]);
                segment.points.push(points[3]);
            }

            Ok(())
        } else {
            Err(Error::PathNotStarted)
        }
    }

    #[must_use]
    pub fn cursor(&self) -> Option<Point> {
        self.segments.last().map(|s| *s.points.last().unwrap())
    }

    pub fn build(self) -> Result<Path, Error> {
        Ok(Path {
            segments: self.segments,
        })
    }

    fn line_as_cubic(p0: Point, p3: Point) -> [Point; 4] {
        let diff = p3 - p0;
        let p1 = diff * 0.25;
        let p2 = diff * 0.75;

        [p0, p1.into(), p2.into(), p3]
    }
}
