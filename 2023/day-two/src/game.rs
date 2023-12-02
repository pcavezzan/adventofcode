use regex::Regex;

pub struct Game {
    pub id: i32,
    pub power: i32,
}

impl Game {
    pub fn parse(line: &str) -> Self {
        let game_regex = Regex::new(r"Game ([0-9]+)").unwrap();
        let caps = game_regex.captures(line).unwrap();
        let game_id: i32 = *&caps[1].parse().unwrap();
        let min_red = get_max_of_color_possible(line, "red");
        let min_green = get_max_of_color_possible(line, "green");
        let min_blue = get_max_of_color_possible(line, "blue");
        let power = min_red * min_blue * min_green;
        return Game {
            id: game_id,
            power,
        };
    }
}

const PREFIX_COLOR_REGEX: &str = "([0-9]+) ";

fn get_max_of_color_possible(line_as_str: &str, color: &str) -> i32 {
    let regex_expression = format!("{PREFIX_COLOR_REGEX}{color}");
    let blue_regex = Regex::new(regex_expression.as_str()).unwrap();
    let option = blue_regex.captures_iter(line_as_str);
    let result = option.into_iter().map(
        |c| {
            let x = &c[1];
            let x1: i32 = x.parse().unwrap();
            return x1;
        }
    ).max().unwrap();
    return result;
}