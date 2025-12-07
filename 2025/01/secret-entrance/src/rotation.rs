use regex::Regex;

pub trait Spatial {
    fn distance(&self) -> i16;
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Rotation {
    Left { d: i16 },
    Right { d: i16 }
}

impl Rotation {
    pub fn parse(s: &str) -> Self {
        let captures = Regex::new(r"^([LR])(\d+)$").unwrap().captures(s).unwrap();
        match captures.get(1).unwrap().as_str() {
            "L" => Rotation::Left { d: captures.get(2).unwrap().as_str().parse::<i16>().unwrap() },
            "R" => Rotation::Right { d: captures.get(2).unwrap().as_str().parse::<i16>().unwrap() },
            _ => panic!("Unknown rotation: {}", s)
        }
    }
}

impl Spatial for Rotation {
    fn distance(&self) -> i16 {
        match self {
            Rotation::Left{ d } => -*d,
            Rotation::Right{ d } => *d,
        }
    }
}