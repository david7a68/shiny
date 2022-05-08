use std::hash::Hash;

use crate::math::cmp::ApproxEq;

use super::{bezier::CubicSlice, point::Point};

#[derive(Clone)]
pub struct Path {
    pub segments: Vec<Segment>,
    pub x: Vec<f32>,
    pub y: Vec<f32>,
}

impl Path {
    pub fn iter(&self) -> SegmentIter {
        SegmentIter {
            path: self,
            segment_idx: 0,
            point_offset: 0,
        }
    }
}

#[derive(Clone, Copy, Hash)]
#[repr(transparent)]
pub struct Segment {
    pub length: u16,
}

pub struct SegmentIter<'a> {
    path: &'a Path,
    segment_idx: usize,
    point_offset: usize,
}

impl<'a> Iterator for SegmentIter<'a> {
    type Item = CurveIter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.segment_idx < self.path.segments.len() {
            let offset = self.point_offset;
            let segment = &self.path.segments[self.segment_idx];

            self.segment_idx += 1;
            self.point_offset += segment.length as usize;

            Some(CurveIter::over_points(
                &self.path.x[offset as usize..offset + segment.length as usize],
                &self.path.y[offset as usize..offset + segment.length as usize],
            ))
        } else {
            None
        }
    }
}

pub struct CurveIter<'a> {
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
    TooManyCurves,
}

#[derive(Default)]
pub struct Builder {
    segments: Vec<Segment>,
    current: Option<Segment>,
    x: Vec<f32>,
    y: Vec<f32>,
    num_curves: u16,
}

impl Builder {
    pub fn move_to(&mut self, point: Point) {
        self.x.push(point.x);
        self.y.push(point.y);
        self.current = Some(Segment { length: 1 });
    }

    pub fn line_to(&mut self, point: Point) -> Result<(), Error> {
        let mut current = self.current.as_mut().ok_or(Error::PathNotStarted)?;

        let points = Self::line_as_cubic(
            *self.x.last().unwrap(),
            *self.y.last().unwrap(),
            point.x,
            point.y,
        );
        self.x.extend(&points[0][1..]);
        self.y.extend(&points[1][1..]);
        current.length += 3;
        self.num_curves.checked_add(1).ok_or(Error::TooManyCurves)?;

        Ok(())
    }

    pub fn add_cubic(&mut self, p1: Point, p2: Point, p3: Point) -> Result<(), Error> {
        let mut current = self.current.as_mut().ok_or(Error::PathNotStarted)?;

        self.x.extend(&[p1.x, p2.x, p3.x]);
        self.y.extend(&[p1.y, p2.y, p3.y]);
        current.length += 3;
        self.num_curves.checked_add(1).ok_or(Error::TooManyCurves)?;

        Ok(())
    }

    pub fn close(&mut self) -> Result<(), Error> {
        let mut current = self.current.take().ok_or(Error::PathNotStarted)?;

        let first_x: f32 = self.x[self.x.len() - current.length as usize];
        let first_y: f32 = self.y[self.y.len() - current.length as usize];
        let last_x = self.x[self.x.len() - 1];
        let last_y = self.y[self.y.len() - 1];

        if !(first_x.approx_eq(&last_x) && first_y.approx_eq(&last_y)) {
            let points = Self::line_as_cubic(last_x, last_y, first_x, first_y);
            self.x.extend(&points[0][1..]);
            self.y.extend(&points[1][1..]);
            current.length += 3;
            self.num_curves.checked_add(1).ok_or(Error::TooManyCurves)?;
        }

        self.segments.push(current);

        Ok(())
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
