use regex::Regex;

pub trait Spatial {
    fn distance(&self) -> i8;
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Rotation {
    Left{ d: i8 }, Right { d: i8 }
}

impl Rotation {
    pub fn parse(s: &str) -> Self {
        let captures = Regex::new(r"^([LR])(\d+)$").unwrap().captures(s).unwrap();
        match captures.get(1).unwrap().as_str() {
            "L" => Rotation::Left { d: captures.get(2).unwrap().as_str().parse::<i8>().unwrap() },
            "R" => Rotation::Right { d: captures.get(2).unwrap().as_str().parse::<i8>().unwrap() },
            _ => panic!("Unknown rotation: {}", s)
        }
    }
}

impl Spatial for Rotation {
    fn distance(&self) -> i8 {
        match self {
            Rotation::Left{ d } => -*d,
            Rotation::Right{ d } => *d,
        }
    }
}