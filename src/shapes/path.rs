use crate::math::cmp::ApproxEq;

use super::{bezier::CubicSlice, point::Point};

pub struct Path {
    pub segments: Vec<Segment>,
    pub points: Vec<Point>,
}

impl Path {
    pub fn iter(&self) -> SegmentIter {
        SegmentIter {
            path: self,
            index: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Segment {
    start: u32,
    end: u32,
}

pub struct SegmentIter<'a> {
    path: &'a Path,
    index: u32,
}

impl<'a> SegmentIter<'a> {
    pub fn iter(&self) -> CurveIter {
        let segment = self.path.segments[self.index as usize];
        CurveIter {
            points: &self.path.points[segment.start as usize..segment.end as usize],
            index: 0,
        }
    }
}

impl<'a> Iterator for SegmentIter<'a> {
    type Item = CurveIter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.path.segments.len() as u32 {
            let segment = &self.path.segments[self.index as usize];
            self.index += 1;
            Some(CurveIter {
                points: &self.path.points[segment.start as usize..(segment.end as usize)],
                index: 0,
            })
        } else {
            None
        }
    }
}

pub struct CurveIter<'a> {
    points: &'a [Point],
    index: u32,
}

impl<'a> Iterator for CurveIter<'a> {
    type Item = CubicSlice<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index as usize + 3 < self.points.len() {
            let slice = &self.points[self.index as usize..(self.index + 4) as usize];
            self.index += 3;
            Some(CubicSlice::new(slice.try_into().unwrap()))
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
    points: Vec<Point>,
}

impl Builder {
    pub fn move_to(&mut self, point: Point) {
        if let Some(previous) = self.segments.last_mut() {
            previous.end = self.points.len() as u32;
        }

        self.points.push(point);
        self.segments.push(Segment {
            start: self.points.len() as u32 - 1,
            end: self.points.len() as u32,
        });
    }

    pub fn line_to(&mut self, point: Point) -> Result<(), Error> {
        if !self.segments.is_empty() {
            let points = Self::line_as_cubic(*self.points.last().unwrap(), point);
            self.points.extend(&points[1..]);
            Ok(())
        } else {
            Err(Error::PathNotStarted)
        }
    }

    pub fn add_cubic(&mut self, p1: Point, p2: Point, p3: Point) -> Result<(), Error> {
        if !self.segments.is_empty() {
            self.points.extend(&[p1, p2, p3]);
            Ok(())
        } else {
            Err(Error::PathNotStarted)
        }
    }

    pub fn close(&mut self) -> Result<(), Error> {
        if let Some(segment) = self.segments.last_mut() {
            let first_point = self.points[segment.start as usize];
            if !first_point.approx_eq(self.points.last().unwrap()) {
                let points = Self::line_as_cubic(*self.points.last().unwrap(), first_point);
                self.points.extend(&points[1..]);
            }
            segment.end = self.points.len() as u32;
            Ok(())
        } else {
            Err(Error::PathNotStarted)
        }
    }

    pub fn cursor(&self) -> Option<Point> {
        self.points.last().cloned()
    }

    pub fn build(self) -> Result<Path, Error> {
        Ok(Path {
            segments: self.segments,
            points: self.points,
        })
    }

    fn line_as_cubic(p0: Point, p3: Point) -> [Point; 4] {
        let diff = p3 - p0;
        let p1 = diff * 0.25;
        let p2 = diff * 0.75;
        [p0, p1.into(), p2.into(), p3]
    }
}
