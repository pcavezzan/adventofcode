#[derive(PartialEq, Debug)]
pub enum Rotation {
    Left, Right
}

impl Rotation {
    pub fn parse(s: &str) -> Self {
        match s {
            "L" => Rotation::Left,
            "R" => Rotation::Right,
            _ => panic!("Unknown rotation: {}", s)
        }
    }
}