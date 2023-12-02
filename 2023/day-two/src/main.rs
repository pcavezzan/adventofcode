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
        let mut sum_game_id = 0;
        for line in lines {
            if let Ok(ip) = line {
                let game = Game::parse(ip.as_str());
                let game_id = game.id;
                if game.is_possible() {
                    println!("{game_id}");
                    sum_game_id += game_id;
                }
            }
        }
        println!("What is the sum of the IDs of those games? {}", sum_game_id);
    } else {}
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}