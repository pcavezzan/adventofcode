module puzzle

fn test_new_instance() {
	content := '190: 10 19\n3267: 81 40 27'

	game := from_string(content)

	assert game.equations.len == 2
	assert game.equations[0].test_value == 190
	assert game.equations[0].numbers == [10, 19]
	assert game.equations[1].test_value == 3267
	assert game.equations[1].numbers == [81, 40, 27]
}

fn test_equations_can_be_made_when_test_value_is_numbers_times() {
	game := Game{
		equations: [
			Equation{
				test_value: 190
				numbers:    [10, 19]
			},
		]
	}

	equations := game.solve_equations()

	assert equations == 190
}

fn test_equations_can_be_made_when_test_value_is_numbers_times_with_sum() {
	game := Game{
		equations: [
			Equation{
				test_value: 3267
				numbers:    [81, 40, 27]
			},
		]
	}

	equations := game.solve_equations()

	assert equations == 3267
}

fn test_solve_equations_return_sum_of_equations_that_could_be_made() {
	game := Game{
		equations: [
			Equation{
				test_value: 190
				numbers:    [10, 19]
			},
			Equation{
				test_value: 3267
				numbers:    [81, 40, 27]
			},
			Equation{
				test_value: 83
				numbers:    [17, 5]
			},
			Equation{
				test_value: 156
				numbers:    [15, 6]
			},
			Equation{
				test_value: 7290
				numbers:    [6, 8, 6, 15]
			},
			Equation{
				test_value: 161011
				numbers:    [16, 10, 13]
			},
			Equation{
				test_value: 192
				numbers:    [17, 8, 14]
			},
			Equation{
				test_value: 21037
				numbers:    [9, 7, 18, 13]
			},
			Equation{
				test_value: 292
				numbers:    [11, 6, 16, 20]
			},
		]
	}

	equations := game.solve_equations()

	assert equations == 3749
}
