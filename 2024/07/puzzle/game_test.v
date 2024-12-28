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

fn test_equations_can_be_made() {
	assert true == false
}
