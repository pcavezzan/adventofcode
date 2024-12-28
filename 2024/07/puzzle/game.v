module puzzle

struct Equation {
	test_value int
	numbers    []int
}

struct Game {
	equations []Equation
}

pub fn from_string(content string) Game {
	mut equations := []Equation{}
	for line in content.split_into_lines() {
		split := line.split(':')
		test_value := split[0].int()
		values_as_string := split[1].trim_space().split(' ')
		mut numbers := []int{}
		for value_as_string in values_as_string {
			numbers << value_as_string.int()
		}
		equation := Equation{test_value, numbers}
		equations << equation
	}
	return Game{equations}
}
