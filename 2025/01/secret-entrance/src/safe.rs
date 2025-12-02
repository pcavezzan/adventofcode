
struct Safe {
    n: Vec<i8>,
    arrow: i8
}

impl Safe {
    pub fn new() -> Self {
        let mut default = vec![];
        for i in 0..100 {
            default.push(i);
        }
        Self { n: default, arrow: 0 }
    }

    pub fn turn(&mut self) {
        self.click()
    }

    fn click(&mut self) {
        self.arrow = self.arrow + 1;
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
    fn should_increment_arrow_value() {
        let mut d = Safe::new();

        d.click();

        assert_eq!(1, d.arrow);
    }

    #[test]
    fn should_turn_dial_by_one_step_on_turn() {
        let mut d = Safe::new();

        d.turn();

        assert_eq!(1, d.arrow);
    }
}