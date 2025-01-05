use day08::puzzle::Puzzle;

fn main() {
    let file_path = String::from("./input.txt");
    let puzzle = Puzzle::from_file(file_path);
    let anti_nodes = puzzle.find_anti_nodes();
    println!("Found {:?} anti nodes", anti_nodes.len());
}
