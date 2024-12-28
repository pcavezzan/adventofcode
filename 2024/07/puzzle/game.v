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

pub fn (game Game) solve_equations() int {
	mut result := 0
	for equation in game.equations {
		mut time := equation.numbers[0]
		mut sum := equation.numbers[0]
		mut mix_start_with_time := equation.numbers[0]
		mut mix_start_with_sum := equation.numbers[0]

		for i := 1; i < equation.numbers.len; i++ {
			time *= equation.numbers[i]
			sum += equation.numbers[i]
			if i % 2 == 0 {
				mix_start_with_sum *= equation.numbers[i]
				mix_start_with_time += equation.numbers[i]
			} else {
				mix_start_with_sum += equation.numbers[i]
				mix_start_with_time *= equation.numbers[i]
			}
		}

		if equation.test_value == time || equation.test_value == sum
			|| equation.test_value == mix_start_with_time
			|| equation.test_value == mix_start_with_sum {
			result += equation.test_value
		}
	}
	return result
}
