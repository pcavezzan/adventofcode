module main

import os

fn main() {
	input_file_content := os.read_file('./input.txt')!
	println(input_file_content)
}
