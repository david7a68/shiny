use crate::shapes::point::Point;

pub enum Change {
    Replace {
        curve: usize,
        first_point: usize,
        last_point: usize,
    },
}

pub struct ChangeList {
    points: Vec<Point>,
    changes: Vec<Change>,
}
