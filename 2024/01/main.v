module main

import math

fn main() {
	left := [3, 4, 2, 1, 3, 3]
	right := [4, 3, 5, 3, 9, 3]

	left_sorted := left.sorted()
	right_sorted := right.sorted()
	mut distances := []int{len: left.len}
	mut total := 0
	for i in 0 .. left.len {
		distance := math.abs(left_sorted[i] - right_sorted[i])
		distances[i] = distance
		total += distance
	}
	println('Distances: ' + distances.str())
	println('Total: ' + total.str())
}
