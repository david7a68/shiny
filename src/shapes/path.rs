use std::hash::Hash;

use crate::math::cmp::ApproxEq;

use super::{bezier::CubicSlice, point::Point};

pub struct Path {
    pub segments: Vec<Segment>,
    // TODO: Convert this into two vecs, one for x, and one for y. This will
    // allow us to do some more & better work with SIMD such as faster bounding
    // box computations and curve evalution (if CubicBezier & CubicSlice are
    // switched too).
    pub x: Vec<f32>,
    pub y: Vec<f32>,
}

impl Path {
    pub fn iter(&self) -> SegmentIter {
        SegmentIter {
            path: self,
            index: 0,
        }
    }
}

#[derive(Clone, Copy, Hash)]
pub struct Segment {
    pub start: u32,
    pub end: u32,
}

pub struct SegmentIter<'a> {
    path: &'a Path,
    index: u32,
}

impl<'a> SegmentIter<'a> {
    pub fn iter(&self) -> CurveIter {
        let segment = self.path.segments[self.index as usize];
        CurveIter {
            x: &self.path.x[segment.start as usize..segment.end as usize],
            y: &self.path.y[segment.start as usize..segment.end as usize],
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
            Some(CurveIter::over_points(
                &self.path.x[segment.start as usize..(segment.end as usize)],
                &self.path.y[segment.start as usize..(segment.end as usize)],
            ))
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
    x: Vec<f32>,
    y: Vec<f32>,
}

impl Builder {
    pub fn move_to(&mut self, point: Point) {
        if let Some(previous) = self.segments.last_mut() {
            previous.end = self.x.len() as u32;
        }

        self.x.push(point.x);
        self.y.push(point.y);
        self.segments.push(Segment {
            start: self.x.len() as u32 - 1,
            end: self.x.len() as u32,
        });
    }

    pub fn line_to(&mut self, point: Point) -> Result<(), Error> {
        if !self.segments.is_empty() {
            let points = Self::line_as_cubic(
                *self.x.last().unwrap(),
                *self.y.last().unwrap(),
                point.x,
                point.y,
            );
            self.x.extend(&points[0][1..]);
            self.y.extend(&points[1][1..]);
            Ok(())
        } else {
            Err(Error::PathNotStarted)
        }
    }

    pub fn add_cubic(&mut self, p1: Point, p2: Point, p3: Point) -> Result<(), Error> {
        if !self.segments.is_empty() {
            self.x.extend(&[p1.x, p2.x, p3.x]);
            self.y.extend(&[p1.y, p2.y, p3.y]);
            Ok(())
        } else {
            Err(Error::PathNotStarted)
        }
    }

    pub fn close(&mut self) -> Result<(), Error> {
        if let Some(segment) = self.segments.last_mut() {
            let first_x = self.x[segment.start as usize];
            let first_y = self.y[segment.start as usize];
            let last_x = self.x.last().unwrap();
            let last_y = self.y.last().unwrap();
            if !(first_x.approx_eq(last_x) && first_y.approx_eq(last_y)) {
                let points = Self::line_as_cubic(*last_x, *last_y, first_x, first_y);
                self.x.extend(&points[0][1..]);
                self.y.extend(&points[1][1..]);
            }
            segment.end = self.x.len() as u32;
            Ok(())
        } else {
            Err(Error::PathNotStarted)
        }
    }

    pub fn cursor(&self) -> Option<Point> {
        self.x
            .last()
            .zip(self.y.last())
            .map(|(x, y)| Point::new(*x, *y))
    }

    pub fn build(self) -> Result<Path, Error> {
        Ok(Path {
            segments: self.segments,
            x: self.x,
            y: self.y,
        })
    }

    fn line_as_cubic(x0: f32, y0: f32, x3: f32, y3: f32) -> [[f32; 4]; 2] {
        let dx = x3 - x0;
        let dy = y3 - y0;

        let x1 = x0 + dx * 0.25;
        let y1 = y0 + dy * 0.25;
        let x2 = x0 + dx * 0.75;
        let y2 = y0 + dy * 0.75;

        [[x0, x1, x2, x3], [y0, y1, y2, y3]]
    }
}
