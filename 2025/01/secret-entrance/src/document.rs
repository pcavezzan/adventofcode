use crate::rotation::Rotation;

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
}


#[cfg(test)]
mod tests {
    use crate::document::Document;
    use crate::rotation::Rotation;

    #[test]
    fn should_parse_document_from_string() {
        let d = Document::parse("L\nR");

        assert_eq!(2, d.seq_rotations.len());
        assert_eq!(Rotation::Left, d.seq_rotations[0]);
        assert_eq!(Rotation::Right, d.seq_rotations[1]);
    }
}