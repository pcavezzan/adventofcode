mod game;

use game::Game;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut sum = 0;
        for line in lines {
            if let Ok(ip) = line {
                let game = Game::parse(ip.as_str());
                let power = game.power;
                sum += power;
            }
        }
        println!("What is the sum of the IDs of those games? {}", sum);
    } else {}
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}