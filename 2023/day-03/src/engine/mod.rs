mod engine_part;

use crate::engine::engine_part::EnginePart;
use crate::file::read_lines;

pub struct Engine{
    lines: Vec<Vec<EnginePart>>
}

impl Engine {

    pub fn part_numbers(self) -> u8 {
        for line in self.lines {

        }
        return 0
    }

    pub fn parse(file_name: &str) -> Option<Self> {
        if let Ok(lines) = read_lines(file_name) {
            // Consumes the iterator, returns an (Optional) String
            let mut engine_lines: Vec<Vec<EnginePart>> = Vec::new();
            for line in lines {
                if let Ok(content) = line {
                    engine_lines.push(EnginePart::parse(content));
                    return Some(
                        Engine {
                            lines: engine_lines
                        }
                    );
                }
            }
        }
        return None;
    }

}