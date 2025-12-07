use crate::rotation::{Rotation, Spatial};

pub struct Safe {
    arrow: i8,
    password: Option<i16>,
    over_passed_zero_count: Option<i16>
}

impl Safe {

    pub fn with_arrow(arrow: i8) -> Self {
        Self { arrow, password: None, over_passed_zero_count: None }
    }

    pub fn turn(&mut self, rotation: Rotation) -> i8 {
        let grid_size: i16 = 100;
        let max_pos = grid_size - 1;
        let min_pos = 0;
        let mut distance = rotation.distance();
        let ratio_how_far = distance / grid_size;
        distance = distance - (ratio_how_far * grid_size);
        let mut nex_post: i16 = self.arrow as i16;
        nex_post = nex_post + distance;
        let mut over_passed_zero_count = nex_post != 0 && ratio_how_far >= 1;
        let mut number_of_times_over_passed_zero = ratio_how_far;
        if nex_post != 0 {
            if nex_post < min_pos {
                nex_post = grid_size + (nex_post - min_pos);
                over_passed_zero_count = self.arrow > 0;
            }

            if nex_post > max_pos {
                nex_post = min_pos + (nex_post - grid_size);
                over_passed_zero_count = self.arrow > 0;
            }
        }

        if number_of_times_over_passed_zero == 0 {
            number_of_times_over_passed_zero = 1;
        }

        if nex_post == 0 {
            self.password = Some(self.password.unwrap_or(0) + 1);
            self.over_passed_zero_count = Some(self.over_passed_zero_count.unwrap_or(0) + number_of_times_over_passed_zero);
            over_passed_zero_count = false;
        }

        if over_passed_zero_count {
            self.over_passed_zero_count = Some(self.over_passed_zero_count.unwrap_or(0) + number_of_times_over_passed_zero);
        }

        self.arrow = nex_post as i8;
        self.arrow
    }

    pub fn password(&self) -> Option<i16> {
        self.password
    }

    pub fn new_security_protocol_password(&self) -> Option<i16> {
        self.over_passed_zero_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_safe_with_default_values() {
        let d = Safe::with_arrow(0);

        // check dial values
        assert_eq!(0, d.arrow);
        assert_eq!(None, d.password);
    }

    #[test]
    fn should_create_safe_with_initial_arrow_value() {
        let d = Safe::with_arrow(11);

        assert_eq!(11, d.arrow);
    }

    #[test]
    fn should_turn_dial_by_one_step_on_turn_right() {
        let mut d = Safe::with_arrow(0);

        d.turn(Rotation::Right { d: 1 });

        assert_eq!(1, d.arrow);
    }

    #[test]
    fn should_turn_dial_by_two_steps_on_turn_left_from_zero() {
        let mut d = Safe::with_arrow(0);

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