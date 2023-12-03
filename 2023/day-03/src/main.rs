use std::str::FromStr;
use crate::engine::Engine;

mod file;
mod engine;

fn main() {
    if let Some(engine) = Engine::parse("./test.txt") {
        // Consumes the iterator, returns an (Optional) String
        let sum = engine.part_numbers();
        println!("What is the sum of all of the part numbers in the engine schematic? {}", sum);
    }
}