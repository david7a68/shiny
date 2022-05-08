use crate::shapes::path::Path;

#[derive(Debug)]
struct Replace {
    position: u16,
    segment_id: u16,
    first_point: u16,
    one_past_last_point: u16,
}

#[derive(Default)]
pub struct ChangeList {
    x: Vec<f32>,
    y: Vec<f32>,
    changes: Vec<Replace>,
}

impl ChangeList {
    pub fn clear(&mut self) {
        self.x.clear();
        self.y.clear();
        self.changes.clear();
    }

    pub fn replace<F>(&mut self, segment: u16, position: u16, mut f: F)
    where
        F: FnMut(&mut Vec<f32>, &mut Vec<f32>),
    {
        let start = self.x.len();
        (f)(&mut self.x, &mut self.y);

        self.changes.push(Replace {
            position,
            segment_id: segment,
            first_point: start.try_into().unwrap(),
            one_past_last_point: self.x.len().try_into().unwrap(),
        });
    }

    pub fn apply(&mut self, path: &mut Path) {
        println!("Applying chages {:?}", &self.changes);

        self.changes.sort_by(|a, b| a.position.cmp(&b.position));

        #[cfg(debug_assertions)]
        for i in 1..self.changes.len() {
            let prev = &self.changes[i - 1];
            let this = &self.changes[i];
            // Check that there are no overlapping replacements
            assert!(prev.position + 3 <= this.position);
            // Check that segment ids are increasing
            assert!(prev.segment_id <= this.segment_id);
        }

        let mut offset: usize = 0;

        for change in &self.changes {
            // grab the new points
            let position = change.position as usize + offset;
            let new_x = &self.x[change.first_point.into()..change.one_past_last_point.into()];
            let new_y = &self.y[change.first_point.into()..change.one_past_last_point.into()];

            // make sure that the first and last point are the same as on the old curve
            debug_assert_eq!(path.x[position], new_x[0]);
            debug_assert_eq!(path.y[position], new_y[0]);
            debug_assert_eq!(path.x[position + 3], new_x[new_x.len() - 1]);
            debug_assert_eq!(path.y[position + 3], new_y[new_y.len() - 1]);

            // splice the new points in, replacing the old curve
            path.x
                .splice(position..=position + 3, new_x.iter().cloned());
            path.y
                .splice(position..=position + 3, new_y.iter().cloned());

            path.segments[change.segment_id as usize].length +=
                TryInto::<u16>::try_into(new_x.len() - 4).unwrap();
            offset += new_x.len() - 4;
        }

        self.changes.clear();
        self.x.clear();
        self.y.clear();
    }
}

#[cfg(test)]
mod tests {
    use crate::shapes::path::Segment;

    use super::*;

    #[test]
    fn replace() {
        let mut path = Path {
            segments: vec![Segment { length: 6 }],
            x: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0],
            y: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0],
        };
        let mut change_list = ChangeList::default();

        change_list.replace(0, 3, |x, y| {
            x.extend(&[4.0, 100.0, 100.0, 7.0]);
            y.extend(&[4.0, 100.0, 100.0, 7.0]);
        });

        change_list.replace(0, 0, |x, y| {
            x.extend(&[1.0, -2.0, -3.0, -4.0, -5.0, -6.0, 4.0]);
            y.extend(&[1.0, -2.0, -3.0, -4.0, -5.0, -6.0, 4.0]);
        });

        change_list.apply(&mut path);

        assert_eq!(
            path.x,
            vec![1.0, -2.0, -3.0, -4.0, -5.0, -6.0, 4.0, 100.0, 100.0, 7.0]
        );
        assert_eq!(path.x, path.y);
        assert_eq!(path.segments.len(), 1);
        assert_eq!(path.segments[0].length, 9);
    }
}
