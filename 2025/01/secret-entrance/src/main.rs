mod document;
mod rotation;
mod safe;

fn main() {
    println!("Hello, world!");
    let lines = "1\n\
        2\n\
        3"
    .split("\n")
    .map(|s| s.trim().to_string())
    .collect::<Vec<String>>();
    println!("{:?}", lines);
}
