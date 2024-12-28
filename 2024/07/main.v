module main

import os
import puzzle

fn main() {
	input_file_content := os.read_file('./input.txt')!
	println(input_file_content)
	game := puzzle.from_string(input_file_content)
	total_calibration_result := game.solve_equations()
	println('total calibration result: ${total_calibration_result}')
}
