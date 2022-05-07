use crate::shapes::{path::Path};

pub enum Change {
    Replace {
        position: usize,
        segment_id: usize,
        first_point: usize,
        num_points: usize,
    },
}

#[derive(Default)]
pub struct ChangeList {
    x: Vec<f32>,
    y: Vec<f32>,
    changes: Vec<Change>,
}

impl ChangeList {
    pub fn replace<F>(&mut self, segment: u16, curve_start: u16, mut f: F)
    where
        F: FnMut(&mut Vec<f32>, &mut Vec<f32>),
    {
        let position = curve_start.into();

        let start = self.x.len();
        (f)(&mut self.x, &mut self.y);
        let count = self.x.len() - start;

        self.changes.push(Change::Replace {
            position,
            segment_id: segment.into(),
            first_point: start,
            num_points: count,
        });
    }

    pub fn apply(&self, path: &mut Path) {
        todo!()
    }
}
