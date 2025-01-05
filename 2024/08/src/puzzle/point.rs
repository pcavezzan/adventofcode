use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
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

    pub fn anti_node(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            point_type: PointType::AntiNode,
        }
    }

    pub fn is_antenna(&self) -> bool {
        if let PointType::Antenna { frequency: _ } = self.point_type {
            true
        } else {
            false
        }
    }

    pub fn frequency(&self) -> Option<&Frequency> {
        match &self.point_type {
            PointType::Antenna { frequency } => Some(frequency),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum  Frequency {
    Str(char),
    Num(i32),
}

#[derive(Debug, Eq, PartialEq)]
enum PointType {
    Antenna { frequency: Frequency },
    AntiNode,
    Regular,
}
