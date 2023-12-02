use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let example_calibration_value = compute_calibration_value_from_file_content("example.txt");
    println!("sum of calibration values: {}", example_calibration_value);
    let day_one_calibration_value = compute_calibration_value_from_file_content("input.txt");
    println!("sum of calibration values: {}", day_one_calibration_value);
}

fn compute_calibration_value_from_file_content(file_name: &str) -> u32 {
    let path = Path::new(file_name);
    let display = path.display();
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    let calibration_value = match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => sum_calibration_values_of_each_line_from_content_second_part(s)
    };
    calibration_value
}

fn sum_calibration_values_of_each_line_from_content_second_part(content: String) -> u32 {
    // println!("{}", content);
    let lines = content.split("\n");
    let calibration_sum = lines.fold(0, |sum, line| {
        let first_digit = first_digit_of_line_content_second_part(line).unwrap();
        let last_digit = last_digit_of_line_content_second_part(line).unwrap();
        let two_digit = format!("{first_digit}{last_digit}");
        let calibration = two_digit.parse::<u32>().unwrap();
        // println!("item: {line}, first digit: {first_digit}, last digit: {last_digit}, calibration value: {calibration}");
        return sum + calibration;
    });
    return calibration_sum;
}

fn first_digit_of_line_content_second_part(line: &str) -> Option<u32> {
    let mut first_occurrences_of_digit = HashMap::new();
    first_occurrences_of_digit.insert(
        get_first_index_of(line, "one", "1"),
        1u32
    );
    first_occurrences_of_digit.insert(
        get_first_index_of(line, "two", "2"),
        2u32
    );
    first_occurrences_of_digit.insert(
        get_first_index_of(line, "three", "3"),
        3u32
    );
    first_occurrences_of_digit.insert(
        get_first_index_of(line, "four", "4"),
        4u32
    );
    first_occurrences_of_digit.insert(
        get_first_index_of(line, "five", "5"),
        5u32
    );
    first_occurrences_of_digit.insert(
        get_first_index_of(line, "six", "6"),
        6u32
    );
    first_occurrences_of_digit.insert(
        get_first_index_of(line, "seven", "7"),
        7u32
    );
    first_occurrences_of_digit.insert(
        get_first_index_of(line, "eight", "8"),
        8u32
    );
    first_occurrences_of_digit.insert(
        get_first_index_of(line, "nine", "9"),
        9u32
    );
    return  first_occurrences_of_digit.iter()
        .filter(|(k, _)| k >= &&0)
        .min()
        .map(| (_, v) | *v);
}

fn get_first_index_of(line: &str, word: &str, digit: &str) -> i32 {
    line.find(word)
        .map(|index| {
            let digit_index = line.find(digit)
                .or(Some(index))
                .unwrap();
            if digit_index <= index {
                return digit_index
            } else {
                return index
            }
        })
        .or(line.find(digit))
        .map(|index| index as i32)
        .or(Some(-1))
        .unwrap()
}

fn last_digit_of_line_content_second_part(line: &str) -> Option<u32> {
    let mut last_occurrences_of_digit = HashMap::new();
    last_occurrences_of_digit.insert(
        get_last_index_of(line, "one", "1"),
        1u32
    );
    last_occurrences_of_digit.insert(
        get_last_index_of(line, "two", "2"),
        2u32
    );
    last_occurrences_of_digit.insert(
        get_last_index_of(line, "three", "3"),
        3u32
    );
    last_occurrences_of_digit.insert(
        get_last_index_of(line, "four", "4"),
        4u32
    );
    last_occurrences_of_digit.insert(
        get_last_index_of(line, "five", "5"),
        5u32
    );
    last_occurrences_of_digit.insert(
        get_last_index_of(line, "six", "6"),
        6u32
    );
    last_occurrences_of_digit.insert(
        get_last_index_of(line, "seven", "7"),
        7u32
    );
    last_occurrences_of_digit.insert(
        get_last_index_of(line, "eight", "8"),
        8u32
    );
    last_occurrences_of_digit.insert(
        get_last_index_of(line, "nine", "9"),
        9u32
    );
    return  last_occurrences_of_digit.iter()
        .filter(|(k, _)| k >= &&0)
        .max()
        .map(| (_, v) | *v);
}

fn get_last_index_of(line: &str, word: &str, digit: &str) -> i32 {
    line.rfind(word)
        .map(|index| {
            let digit_index = line.rfind(digit)
                .or(Some(index))
                .unwrap();
            if digit_index >= index {
                return digit_index
            } else {
                return index
            }
        })
        .or(line.rfind(digit))
        .map(|index| index as i32)
        .or(Some(-1))
        .unwrap()
}
