use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

fn main() {
    my_own_solution();
}

// Really impressive solution from @timvisee
// See: https://github.com/timvisee/advent-of-code-2024/blob/master/day04a/src/main.rs
fn other_solution() {
    let mut word = [0; 4];
    let map = include_bytes!("../input.txt")
        .split(|&c| c == b'\n')
        .collect::<Vec<_>>();

    println!(
        "{}",
        (0..map[0].len() as isize)
            .flat_map(|x| (0..map.len() as isize).map(move |y| (x, y)))
            .flat_map(|(x, y)| {
                [
                    [(x, y), (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)], // NE
                    [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],             // E
                    [(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)], // SE
                    [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],             // S
                ]
            })
            .filter(|coords| {
                let mut iter = coords.iter().map(|(x, y)| {
                    map.get(*y as usize)
                        .and_then(|row| row.get(*x as usize).copied())
                        .unwrap_or_default()
                });
                word.fill_with(|| iter.next().unwrap_or_default());
                &word == b"XMAS" || &word == b"SAMX"
            })
            .count(),
    );
}

fn my_own_solution() {
    let file = File::open(Path::new("./input.txt")).unwrap();
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content).unwrap();
    //
    println!("Input:\n{}\n", content);
    //
    let lines = content.split("\n").collect::<Vec<&str>>();
    let mut char_matrix: Vec<Vec<String>> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        char_matrix.push(Vec::new());
        for char in line.chars() {
            char_matrix[i].push(char.to_string());
        }
    }
    // Now
    let mut christmas_words = Vec::new();
    for col in 0..char_matrix[0].len() {
        for row in 0..char_matrix.len() {
            if row < char_matrix.len() - 3 {
                let word = format!(
                    "{}{}{}{}",
                    char_matrix[row][col],
                    char_matrix[row+1][col],
                    char_matrix[row+2][col],
                    char_matrix[row+3][col]
                );
                if word == "XMAS" || word == "SAMX" {
                    christmas_words.push(word);
                }
            }

            if col < (char_matrix[0].len() - 3) {
                let word = format!(
                    "{}{}{}{}",
                    char_matrix[row][col],
                    char_matrix[row][col+1],
                    char_matrix[row][col+2],
                    char_matrix[row][col+3]
                );
                if word == "XMAS" || word == "SAMX" {
                    christmas_words.push(word);
                }

                if row < char_matrix.len() - 3 {
                    let word = format!(
                        "{}{}{}{}",
                        char_matrix[row][col],
                        char_matrix[row+1][col+1],
                        char_matrix[row+2][col+2],
                        char_matrix[row+3][col+3]
                    );
                    if word == "XMAS" || word == "SAMX" {
                        christmas_words.push(word);
                    }
                }

                if row > 3 {
                    let word = format!(
                        "{}{}{}{}",
                        char_matrix[row][col],
                        char_matrix[row-1][col+1],
                        char_matrix[row-2][col+2],
                        char_matrix[row-3][col+3]
                    );
                    if word == "XMAS" || word == "SAMX" {
                        christmas_words.push(word);
                    }
                }
            }
        }
    }
    println!("Found {} christmas either written as XMAS or SAMX", christmas_words.len());
}
