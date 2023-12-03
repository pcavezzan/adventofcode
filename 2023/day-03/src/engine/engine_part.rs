use std::io::empty;
use std::ptr::null;
use crate::engine::engine_part::PartType::{DIGIT, DOT, SYMBOL};

pub struct EngineLine {
    parts: Vec<EnginePart>,
    numbers: Vec<Number>
}

#[derive(Clone)]
struct Number {
    value: String,
    start_position: (u8, u8),
    end_position: (u8, u8)
}


pub struct EnginePart {
    value: char,
    engine_part_type: PartType,
    position: u8
}

#[derive(PartialEq, Eq)]
enum PartType {
    SYMBOL,
    DIGIT,
    DOT
}


impl EnginePart {
    pub fn parse(content: String) -> Vec<Self> {
        let mut parts: Vec<Self> = Vec::new();
        for (i, c) in content.as_str().char_indices() {
            let engine_part_type = if c.is_digit(10) {
                DIGIT
            } else if c == '.' {
                DOT
            } else {
                SYMBOL
            };
            parts.push(EnginePart{
                position: i as u8,
                engine_part_type,
                value: c
            });
        }
        return parts
    }
}

impl EngineLine {

    fn parse(line: String, line_number: u8) -> Self {
        let parts= EnginePart::parse(line);
        let mut numbers: Vec<&Number> = Vec::new();
        let mut number_detector: Vec<Number> = Vec::new();
        for i in 0..parts.len() {
            let part = parts.get(i).unwrap();
            if DIGIT == part.engine_part_type {
                if (number_detector.is_empty()) {
                    let new_number = Number {
                        value: part.value.to_string(),
                        start_position: (line_number, i as u8),
                        end_position: (line_number, i as u8),
                    };
                    number_detector.push(new_number)
                } else {
                    let current_number = number_detector.get(0).unwrap();
                    current_number.clone().append(part)
                }
            } else {
                let current_number_detected = number_detector.get(0).unwrap();
                numbers.push(current_number_detected);
            }
        }

        return EngineLine {
            parts,
            numbers: Vec::new()
        }
    }

}

impl Number {
    fn append(mut self, engine_part: &EnginePart) {
        self.value = self.value + String::from(engine_part.value).as_str();
        self.end_position = (self.end_position.0, self.end_position.1 + 1);
    }
}
