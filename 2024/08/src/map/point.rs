use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
    point_type: PointType,
}

impl Point {
    pub fn regular(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            point_type: PointType::Regular,
        }
    }

    pub fn char_antenna(x: i32, y: i32, frequency: char) -> Self {
        Self {
            x,
            y,
            point_type: PointType::Antenna {
                frequency: Frequency::Str(frequency),
            },
        }
    }

    pub fn int_antenna(x: i32, y: i32, frequency: i32) -> Self {
        Self {
            x,
            y,
            point_type: PointType::Antenna {
                frequency: Frequency::Num(frequency),
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum  Frequency {
    Str(char),
    Num(i32),
}

#[derive(Debug, Eq, PartialEq)]
enum PointType {
    Antenna { frequency: Frequency },
    AntiNode,
    Regular,
}
