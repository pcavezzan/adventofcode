module main

import math

fn main() {
	mut reports := [][]int{}
	reports << [7, 6, 4, 2, 1]
	reports << [1, 2, 7, 8, 9]
	reports << [9, 7, 6, 2, 1]
	reports << [1, 3, 2, 4, 5]
	reports << [8, 6, 4, 4, 1]
	reports << [1, 3, 6, 7, 9]

	mut total_safe_detected := 0

	for report in reports {
		mut last_digit := -1
		mut last_increasing := 0
		mut safe := true
		for digit in report {
			if last_digit != -1 && safe {
				difference := digit - last_digit
				increasing := if difference < 0 { -1 } else { 1 }
				abs_difference := math.abs(difference)
				if last_increasing != 0 && (increasing != last_increasing || abs_difference > 3) {
					safe = false
				}
				last_increasing = increasing
			}
			last_digit = digit
		}

		if safe {
			total_safe_detected += 1
		}
	}

	println('Number of safe reports: ' + total_safe_detected.str())
}
