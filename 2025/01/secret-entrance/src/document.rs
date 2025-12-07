use crate::rotation::Rotation;
use crate::safe::Safe;

struct Document {
    seq_rotations: Vec<Rotation>,
}


impl Document {
    pub fn new(seq_rotations: Vec<Rotation>) -> Self {
        Self { seq_rotations }
    }


    pub fn parse(s: &str) -> Self {
        let seq_rotations =  s
            .split("\n")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| Rotation::parse(&s))
            .collect::<Vec<Rotation>>();

        Self::new(seq_rotations)
    }

    pub fn from_file(file_path: &str) -> Self {
        let contents = std::fs::read_to_string(file_path)
            .expect(format!("Something went wrong reading the file {file_path}").as_str());
        Self::parse(&contents)
    }

    pub fn apply_on(&self, safe: &mut Safe) -> i8 {
        let mut arrow = 0;
        for r in &self.seq_rotations {
            let rotation = r.clone();
            arrow = safe.turn(rotation);
        }
        arrow
    }

    pub fn find_password(&self, safe: &mut Safe) -> Option<i8> {
        self.apply_on(safe);
        safe.password()
    }
}


#[cfg(test)]
mod tests {
    use crate::document::Document;
    use crate::rotation::Rotation;
    use crate::safe::Safe;

    #[test]
    fn should_parse_document_from_string() {
        let d = Document::parse("L11\nR8");

        assert_eq!(2, d.seq_rotations.len());
        assert_eq!(Rotation::Left{ d: 11 }, d.seq_rotations[0]);
        assert_eq!(Rotation::Right{ d: 8 }, d.seq_rotations[1]);
    }

    #[test]
    fn should_read_document_from_file() {
        let d = Document::from_file("test_input.txt");

        assert_eq!(2, d.seq_rotations.len());
        assert_eq!(Rotation::Left { d: 11 }, d.seq_rotations[0]);
        assert_eq!(Rotation::Right { d: 8 }, d.seq_rotations[1]);
    }

    #[test]
    fn should_apply_document_on_safe() {
        let d = Document::parse("R8\nL19");
        let mut safe = Safe::with_arrow(11);

        let arrow = d.apply_on(&mut safe);

        assert_eq!(0, arrow);
    }


    #[test]
    fn should_turn_left_then_right_from_dial_starts_at_five() {
        let d = Document::parse("L10\nR5");
        let mut safe = Safe::with_arrow(5);

        let arrow = d.apply_on(&mut safe);

        assert_eq!(0, arrow);
    }

    #[test]
    fn should_find_safe_password() {
        let d = Document::parse("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82");
        let mut safe = Safe::with_arrow(50);

        let password = d.find_password(&mut safe);

        assert_eq!(Some(3), password);
    }
}