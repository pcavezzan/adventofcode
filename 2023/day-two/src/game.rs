use regex::Regex;

pub struct Game {
    pub id: i32,
    possible: bool
}

impl Game {
    pub fn parse(line: &str) -> Self {
        let game_regex = Regex::new(r"Game ([0-9]+)").unwrap();
        let caps = game_regex.captures(line).unwrap();
        let game_id: i32 = *&caps[1].parse().unwrap();
        let is_red_possible = is_number_of_color_possible(line, "red", 12);
        let is_green_possible = is_number_of_color_possible(line, "green", 13);
        let is_blue_possible = is_number_of_color_possible(line, "blue", 14);
        return Game {
            id: game_id,
            possible: is_red_possible && is_green_possible && is_blue_possible
        };
    }

    pub fn is_possible(self) -> bool {
        return self.possible
    }
}

const PREFIX_COLOR_REGEX: &str = "([0-9]+) ";

fn is_number_of_color_possible(line_as_str: &str, color: &str, limit: i32) -> bool {
    let regex_expression =  format!("{PREFIX_COLOR_REGEX}{color}");
    let blue_regex = Regex::new(regex_expression.as_str()).unwrap();
    let option = blue_regex.captures_iter(line_as_str);
    let result = option.into_iter().all(
        |c| {
            let x = &c[1];
            let x1: i32 = x.parse().unwrap();
            return x1 <= limit;
        }
    );
    return result
}