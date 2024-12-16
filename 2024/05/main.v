module main

import os
import math

fn main() {
	rules_file_content := read_file('./rules.txt')
	println(rules_file_content)
	update_file_content := read_file('./updates.txt')
	println(update_file_content)

	mut rules := map[int][]int{}
	for line in rules_file_content.split_into_lines() {
		rule := line.split('|')
		name := rule[0].int()
		mut associated_with_name := rules[name] or { []int{} }
		associated_with_name << rule[1].int()
		rules[name] = associated_with_name
	}

	mut total := 0
	for line in update_file_content.split_into_lines() {
		mut valid_line := false
		int_line := line
			.split(',')
			.map(it.int())
		for i := 0; i < int_line.len; i++ {
			int_value := int_line[i]
			int_rules := rules[int_value] or { []int{} }
			if i != int_line.len - 1 {
				for j := i + 1; j < int_line.len; j++ {
					other_int := int_line[j]
					if int_rules.any(it == other_int) {
						valid_line = true
					}
				}
			}

			if i > 0 {
				for j := i - 1; j >= 0; j-- {
					other_int := int_line[j]
					if int_rules.any(it == other_int) {
						valid_line = false
					}
				}
			}

			if !valid_line {
				break
			}
		}

		if valid_line {
			result := math.divide_euclid(int_line.len, 2)
			middle_index := result.quot + (result.rem - 1)
			middle_int_value := int_line[middle_index]
			total += middle_int_value
		}
	}

	println('Result: ${total}')
}

fn read_file(file_name string) string {
	return os.read_file(file_name) or { panic('error reading file ${file_name}') }
}
