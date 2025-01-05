use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::puzzle::point::Point;

mod point;

#[derive(Debug, Eq, PartialEq)]
pub struct Puzzle {
    points: Vec<Vec<Point>>
}

impl Puzzle {
    pub fn from_file(file_path: String) -> Self {
        let file = File::open(file_path).expect("file not found");
        let reader = BufReader::new(file);
        let mut matrix: Vec<Vec<Point>> = Vec::new();
        let mut line_number = 0;
        for line in reader.lines() {
            let mut column = 0;
            let points = line.unwrap()
                .chars()
                .into_iter()
                .map(move |x| {
                    let point = if x.is_alphabetic() {
                        Point::char_antenna(column, line_number, x)
                    } else if x.is_numeric() {
                        Point::int_antenna(column, line_number, x.to_digit(10).unwrap() as i32)
                    } else {
                        Point::regular(column, line_number)
                    };
                    column += 1;
                    point
                })
                .collect::<Vec<Point>>();
            matrix.push(points);
            line_number += 1;
        }
        Self { points: matrix }
    }

    pub fn find_anti_nodes(&self) -> Vec<Point> {
        let mut anti_nodes = Vec::new();
        let mut antennas: Vec<&Point> = vec![];

        for row in self.points.iter() {
            for point in row.iter() {
                if point.is_antenna() {
                    antennas.push(point);
                }
            }
        }

        for i in 0..antennas.len() {
            let reference_antenna = antennas[i];
            for j in i + 1..antennas.len() {
                let antenna_to_compare_with = antennas[j];

                if reference_antenna.frequency() == antenna_to_compare_with.frequency() {
                    let dx = antenna_to_compare_with.x - reference_antenna.x;
                    let dy = antenna_to_compare_with.y - reference_antenna.y;

                    let first_anti_node = Point::anti_node(reference_antenna.x - dx, reference_antenna.y - dy);
                    let second_anti_node = Point::anti_node(antenna_to_compare_with.x + dx, antenna_to_compare_with.y + dy);

                    if self.can_be_put_on_map(&first_anti_node) {
                        anti_nodes.push(first_anti_node);
                    }

                    if self.can_be_put_on_map(&second_anti_node) {
                        anti_nodes.push(second_anti_node);
                    }
                }
            }
        }

        anti_nodes
    }

    fn can_be_put_on_map(&self, point: &Point) -> bool {
        point.x >= 0 && point.y >= 0 && (point.y as usize) < self.points.len() && (point.x as usize) < self.points[0].len()
    }
}


#[cfg(test)]
mod tests {
    use crate::puzzle::Puzzle;
    use crate::puzzle::point::Point;

    #[test]
    fn create_new_map_from_file() {
        let file_path = String::from("./test_data/puzzle.txt");

        let map = Puzzle::from_file(file_path);

        assert_eq!(map, Puzzle {
            points: vec![
                vec![
                    Point::regular(0, 0),
                    Point::int_antenna(1, 0, 0),
                    Point::regular(2, 0),
                ],
                vec![
                    Point::regular(0, 1),
                    Point::regular(1, 1),
                    Point::char_antenna(2, 1, 'A'),
                ],
                vec![
                    Point::regular(0, 2),
                    Point::regular(1, 2),
                    Point::regular(2, 2),
                ],
            ]
        });
    }

    #[test]
    fn should_find_anti_node_in_puzzle() {
        let map = Puzzle {
            points: vec![
                vec![
                    Point::regular(0, 0),
                    Point::regular(1, 0),
                    Point::regular(2, 0),
                    Point::regular(3, 0),
                ],
                vec![
                    Point::regular(0, 1),
                    Point::regular(1, 1),
                    Point::regular(2, 1),
                    Point::regular(3, 1),
                ],
                vec![
                    Point::regular(0, 2),
                    Point::char_antenna(1, 2, 'a'),
                    Point::regular(2, 2),
                    Point::regular(3, 2),
                ],
                vec![
                    Point::regular(0, 3),
                    Point::regular(1, 3),
                    Point::regular(2, 3),
                    Point::regular(3, 3),
                ],
                vec![
                    Point::regular(0, 4),
                    Point::regular(1, 4),
                    Point::char_antenna(2, 4, 'a'),
                    Point::regular(3, 4),
                ],
                vec![
                    Point::regular(0, 5),
                    Point::regular(1, 5),
                    Point::regular(2, 5),
                    Point::regular(3, 5),
                ],
                vec![
                    Point::regular(0, 6),
                    Point::regular(1, 6),
                    Point::regular(2, 6),
                    Point::regular(3, 6),
                ],
            ]
        };

        let anti_node_points = map.find_anti_nodes();

        assert_eq!(
            anti_node_points,
            vec![
                Point::anti_node(0, 0),
                Point::anti_node(3, 6)
            ]
        )
    }
}