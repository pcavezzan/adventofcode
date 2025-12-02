use regex::Regex;

#[derive(PartialEq, Debug)]
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