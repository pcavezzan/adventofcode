use crate::rotation::Rotation;

struct Safe {
    n: Vec<i8>,
    arrow: i8
}

impl Safe {

    pub fn new() -> Self {
        Self::with_arrow(0)
    }

    pub fn with_arrow(arrow: i8) -> Self {
        let mut default = vec![];
        for i in 0..100 {
            default.push(i);
        }
        Self { n: default, arrow }
    }

    pub fn turn(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::Left { d } => {
                self.arrow = 100 + ((self.arrow - d) % 100);
            }
            Rotation::Right { d } => {
                self.arrow = (self.arrow + d) % 100;
            }
        }
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