use crate::math::cmp::ApproxEq;

use super::{bezier::CubicSlice, point::Point};

pub struct Path {
    pub segments: Vec<Segment>,
}

impl Path {
    pub fn iter(&self) -> SegmentIter {
        SegmentIter {
            path: self,
            segment_index: 0,
        }
    }
}

#[derive(Clone, Default)]
pub struct Segment {
    pub x: Vec<f32>,
    pub y: Vec<f32>,
}

impl Segment {
    pub fn first(&self) -> Option<Point> {
        if self.x.is_empty() {
            None
        } else {
            // TODO: index_unchecked?
            Some(Point::new(self.x[0], self.y[0]))
        }
    }

    pub fn last(&self) -> Option<Point> {
        if self.x.is_empty() {
            None
        } else {
            Some(Point::new(
                self.x[self.x.len() - 1],
                self.y[self.y.len() - 1],
            ))
        }
    }
}

pub struct SegmentIter<'a> {
    path: &'a Path,
    segment_index: u32,
}

impl<'a> SegmentIter<'a> {
    pub fn iter(&self) -> CurveIter {
        let segment = &self.path.segments[self.segment_index as usize];
        CurveIter {
            x: &segment.x,
            y: &segment.y,
            index: 0,
        }
    }
}

impl<'a> Iterator for SegmentIter<'a> {
    type Item = CurveIter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.segment_index < self.path.segments.len() as u32 {
            let segment = &self.path.segments[self.segment_index as usize];
            self.segment_index += 1;
            Some(CurveIter::over_points(&segment.x, &segment.y))
        } else {
            None
        }
    }
}

pub struct CurveIter<'a> {
    // points: &'a [Point],
    x: &'a [f32],
    y: &'a [f32],
    index: u32,
}

impl<'a> CurveIter<'a> {
    pub fn over_points(x: &'a [f32], y: &'a [f32]) -> CurveIter<'a> {
        CurveIter { x, y, index: 0 }
    }
}

impl<'a> Iterator for CurveIter<'a> {
    type Item = CubicSlice<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index as usize + 3 < self.x.len() {
            let x = &self.x[self.index as usize..(self.index + 4) as usize];
            let y = &self.y[self.index as usize..(self.index + 4) as usize];
            self.index += 3;
            Some(CubicSlice::new(
                x.try_into().unwrap(),
                y.try_into().unwrap(),
            ))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Error {
    PathNotStarted,
}

#[derive(Default)]
pub struct Builder {
    segments: Vec<Segment>,
    current: Option<Segment>,
}

impl Builder {
    pub fn move_to(&mut self, point: Point) {
        if let Some(current) = self.current.take() {
            self.segments.push(current);
        }

        self.current = Some(Segment {
            x: vec![point.x],
            y: vec![point.y],
        });
    }

    pub fn line_to(&mut self, point: Point) -> Result<(), Error> {
        let segment = self.current.as_mut().ok_or(Error::PathNotStarted)?;
        let points = Self::line_as_cubic(segment.last().unwrap(), point);
        segment.x.extend(&points[0][1..]);
        segment.y.extend(&points[1][1..]);
        Ok(())
    }

    pub fn add_cubic(&mut self, p1: Point, p2: Point, p3: Point) -> Result<(), Error> {
        let segment = self.current.as_mut().ok_or(Error::PathNotStarted)?;
        segment.x.extend(&[p1.x, p2.x, p3.x]);
        segment.y.extend(&[p1.y, p2.y, p3.y]);
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), Error> {
        let mut segment = self.current.take().ok_or(Error::PathNotStarted)?;
        let first = segment.first().unwrap();
        let last = segment.last().unwrap();

        if !first.approx_eq(&last) {
            let points = Self::line_as_cubic(last, first);
            segment.x.extend(&points[0][1..]);
            segment.y.extend(&points[1][1..]);
        }

        self.segments.push(segment);
        Ok(())
    }

    pub fn cursor(&self) -> Option<Point> {
        self.current.as_ref().map(|s| s.last().unwrap())
    }

    pub fn build(self) -> Result<Path, Error> {
        Ok(Path {
            segments: self.segments,
        })
    }

    fn line_as_cubic(p0: Point, p3: Point) -> [[f32; 4]; 2] {
        let d = p3 - p0;
        let p1 = p0 + d * 0.25;
        let p2 = p3 + d * 0.75;
        [[p0.x, p1.x, p2.x, p3.x], [p0.y, p1.y, p2.y, p3.y]]
    }
}
