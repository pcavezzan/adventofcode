use crate::rotation::{Rotation, Spatial};

pub struct Safe {
    n: Vec<i8>,
    arrow: i8,
    password: Option<i8>
}

impl Safe {

    pub fn new() -> Self {
        Self::with_arrow(0)
    }

    pub fn with_arrow(arrow: i8) -> Self {
        let mut default = vec![100; 0];
        for i in 0..100 {
            default.push(i);
        }
        Self { n: default, arrow, password: None }
    }

    pub fn turn(&mut self, rotation: Rotation) -> i8 {
        let grid_size = 100;
        let max_pos = grid_size - 1;
        let min_pos = 0;
        let mut distance = rotation.distance();
        let ratio_how_far = distance / max_pos;
        distance = distance - (ratio_how_far * grid_size);
        let mut nex_post: i16 = self.arrow as i16;
        nex_post = nex_post + distance;
        if nex_post < min_pos {
            nex_post = grid_size + (nex_post - min_pos);
        }

        if nex_post > max_pos {
            nex_post = min_pos + (nex_post - grid_size);
        }

        if nex_post == 0 {
            self.password = Some(self.password.unwrap_or(0) + 1);
        }

        self.arrow = nex_post as i8;
        self.arrow
    }

    pub fn password(&self) -> Option<i8> {
        self.password
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_safe_with_default_values() {
        let d = Safe::new();

        // check dial values
        assert_eq!(0, d.n[0] );
        assert_eq!(99, d.n[99]);
        // check arrow value
        assert_eq!(0, d.arrow);
    }

    #[test]
    fn should_create_safe_with_initial_arrow_value() {
        let d = Safe::with_arrow(11);

        // check dial values
        assert_eq!(0, d.n[0] );
        assert_eq!(99, d.n[99]);
        // check arrow value
        assert_eq!(11, d.arrow);
    }

    #[test]
    fn should_turn_dial_by_one_step_on_turn_right() {
        let mut d = Safe::new();

        d.turn(Rotation::Right { d: 1 });

        assert_eq!(1, d.arrow);
    }

    #[test]
    fn should_turn_dial_by_two_steps_on_turn_left_from_zero() {
        let mut d = Safe::new();

        d.turn(Rotation::Left { d: 1 });

        assert_eq!(99, d.arrow);
    }


    #[test]
    fn should_turn_dial_by_one_steps_on_turn_right_from_ninety_nine() {
        let mut d = Safe::with_arrow(99);

        d.turn(Rotation::Right { d: 1 });

        assert_eq!(0, d.arrow);
    }
}