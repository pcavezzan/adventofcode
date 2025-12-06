use crate::rotation::Rotation;
use crate::safe::Safe;

struct Document {
    seq_rotations: Vec<Rotation>
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

    pub fn apply_on(&self, safe: &mut Safe) -> i8 {
        let mut arrow = 0;
        for r in &self.seq_rotations {
            let rotation = r.clone();
            arrow = safe.turn(rotation);
        }
        arrow
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
    fn should_apply_document_on_safe() {
        let d = Document::parse("R8\nL19");
        let mut safe = Safe::with_arrow(11);

        let arrow = d.apply_on(&mut safe);

        assert_eq!(0, arrow);
    }
}