use regex::Regex;
use crate::game;

pub struct Game {
    pub id: i32,
    blue: i32,
    red: i32,
    green: i32
}

impl Game {
    pub fn parse(line: &str) -> Self {
        let game_regex = Regex::new(r"Game ([0-9]+)").unwrap();
        let caps = game_regex.captures(line).unwrap();
        let game_id: i32 = *&caps[1].parse().unwrap();
        let number_of_blue = game::parse_number_of_color(line, "blue");
        let number_of_red = game::parse_number_of_color(line, "red");
        let number_of_green = game::parse_number_of_color(line, "green");
        return Game {
            id: game_id,
            blue: number_of_blue,
            red: number_of_red,
            green: number_of_green,
        };
    }

    pub fn is_impossible(self) -> bool {
        if self.red > 12 || self.green > 13 || self.blue > 14 {
            return true
        }
        return false;
    }

    pub fn is_possible(self) -> bool {
        return !self.is_impossible()
    }
}

const PREFIX_COLOR_REGEX: &str = "([0-9]+) ";

fn parse_number_of_color(line_as_str: &str, color: &str) -> i32 {
    let regex_expression =  format!("{PREFIX_COLOR_REGEX}{color}");
    let blue_regex = Regex::new(regex_expression.as_str()).unwrap();
    let option = blue_regex.captures_iter(line_as_str);
    let number_of_blue = option.into_iter().fold(
        0,
        |c, v| {
            let x = &v[1];
            let x1: i32 = x.parse().unwrap();
            return c + x1;
        }
    );
    number_of_blue
}