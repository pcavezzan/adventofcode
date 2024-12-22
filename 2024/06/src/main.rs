use std::fmt::Display;
use crate::Position::Unvisited;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use crate::PuzzleMapError::{Obstacle, Exit, WrongPosition};

#[derive(Clone, Debug)]
struct Point { x: i32, y: i32 }

#[derive(Debug,Clone)]
enum Position {
    Guard { face : char, point: Point },
    Obstacle,
    Unvisited,
    Visited,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::Guard { face, .. } => {
                write!(f, "{}", *face)
            }
            Position::Obstacle { .. } => {
                write!(f, "#")
            }
            Unvisited { .. } => {
                write!(f, ".")
            }
            Position::Visited { .. } => {
                write!(f, "X")
            }
        }
    }
}

impl Position {
    fn from_char_at(c: char, x: i32, y: i32) -> Self {
        match c {
            '^' | '>' | '<' | 'v' => Position::Guard { face : c, point: Point { x, y} },
            '#' => Position::Obstacle,
            _ => Unvisited,
        }
    }

    fn turn_right(&self) -> Position {
        let pos = self.clone();
        match pos {
            Position::Guard { face , point} => match face {
                '^' => Position::Guard { face: '>', point },
                '>' => Position::Guard { face: 'v', point },
                'v' => Position::Guard { face: '<', point },
                '<' => Position::Guard { face: '^', point },
                _ => {
                    panic!("Invalid face '{}'", face);
                }
            },
            _ => pos,
        }
    }

    fn move_forward(&self) -> Position {
        let pos = self.clone();
        match pos {
            Position::Guard { face, point } => {
                match face {
                    '^' => Position::Guard { face, point: Point { x: point.x - 1, y: point.y } },
                    '>' => Position::Guard { face, point: Point { x: point.x, y: point.y + 1 } },
                    'v' => Position::Guard { face, point: Point { x: point.x + 1, y: point.y } },
                    '<' => Position::Guard { face, point: Point { x: point.x, y: point.y - 1 } },
                    _ => {
                        panic!("Invalid face '{}'", face);
                    }
                }
            }
            _ => pos, // other position cannot move
        }
    }
}

#[derive(Debug)]
struct PuzzleMap {
    positions: Vec<Vec<Position>>,
}

#[derive(Debug)]
enum PuzzleMapError {
    Obstacle,
    WrongPosition,
    Exit,
}

impl PuzzleMap {
    fn open(path: &str) -> PuzzleMap {
        let file = File::open(path).expect("file not found");
        let reader = BufReader::new(file);
        let mut map: Vec<Vec<Position>> = Vec::new();
        let mut line_number = 0;
        for line in reader.lines() {
            let mut column = -1;
            let positions = line.unwrap()
                .chars()
                .into_iter()
                .map(move |x| {
                    column += 1;
                    Position::from_char_at(x, line_number, column)
                })
                .collect::<Vec<Position>>();
            map.push(positions);
            line_number += 1;
        }
        PuzzleMap { positions: map }
    }

    fn guard(self: &PuzzleMap) -> Option<Position> {
        for row in self.positions.iter() {
            for position in row.iter() {
                if let Position::Guard { face: _, point: _ } = position {
                    return Some(position.clone());
                }
            }
        }
        None
    }

    fn move_forward(&mut self, position: &Position) -> Result<Position, PuzzleMapError> {
        match *position {
            Position::Guard { face : _, point: Point { x: current_x, y: current_y }} => {
                match position.move_forward() {
                    Position::Guard { face, point: Point { x, y } } => {
                        if x == self.positions[0].len() as i32 || y == self.positions[0].len() as i32 {
                            self.positions[current_x as usize][current_y as usize] = Position::Visited;
                            return Err(Exit)
                        }

                        let next_position = self.positions[x as usize][y as usize].clone();
                        if let Position::Obstacle = next_position {
                            return Err(Obstacle);
                        }
                        self.positions[current_x as usize][current_y as usize] = Position::Visited;
                        let guard = Position::Guard { face, point: Point { x, y } };
                        self.positions[x as usize][y as usize] = guard.clone();
                        Ok(guard)
                    }
                    _ => Err(WrongPosition),
                }
            }
            _ => Err(WrongPosition),
        }
    }

    fn display_guard_path(&self) {
        for row in self.positions.iter() {
            for position in row.iter() {
                print!("{}", position);
            }
            println!();
        }
    }

    fn positions_visited_count(&self) -> i64 {
        let mut count = 0;
        for row in self.positions.iter() {
            for position in row.iter() {
                if let Position::Visited { .. } = position {
                    count += 1;
                }
            }
        }
        count
    }
}

fn main() {
    let mut puzzles = PuzzleMap::open("./map.txt");
    let mut position = puzzles.guard().expect("guard should exist");
    loop {
        let next_position_result = puzzles.move_forward(&position);
        match next_position_result {
            Ok(next_position) => {
                position = next_position;
            },
            Err(Obstacle) => {
                position = position.turn_right();
            }
            Err(WrongPosition) => {
                exit(-1)
            }
            Err(Exit) => {
                break
            }
        };
    }

    puzzles.display_guard_path();
    println!("Number of distinct positions visited by guard: {}", puzzles.positions_visited_count())
}
