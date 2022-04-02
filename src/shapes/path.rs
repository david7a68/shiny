use super::{bezier::CubicSlice, point::Point};

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

#[derive(Clone, Copy)]
struct PathSegment {
    first: usize,
    last: usize,
}

pub struct Builder {
    segments: Vec<PathSegment>,
    points: Vec<Point>,
    path_start_offset: usize,
}

impl Builder {
    #[must_use]
    pub fn new(start: Point) -> Self {
        Self {
            segments: vec![],
            points: vec![start],
            path_start_offset: 0,
        }
    }

    pub fn move_to(&mut self, point: Point) {
        // The span of points that make up the previous subpath.
        let segment = PathSegment {
            first: self.path_start_offset,
            last: self.points.len() - 1,
        };

        self.segments.push(segment);
        self.path_start_offset = self.points.len();
        self.points.push(point);
    }

    pub fn add_cubic(&mut self, p1: Point, p2: Point, p3: Point) {
        self.points.push(p1);
        self.points.push(p2);
        self.points.push(p3);
    }

    #[must_use]
    pub fn build(mut self) -> Path {
        self.segments.push(PathSegment {
            first: self.path_start_offset,
            last: self.points.len() - 1,
        });

        Path {
            segments: self.segments.into_boxed_slice(),
            points: self.points.into_boxed_slice(),
        }
    }
}
