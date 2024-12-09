module main

import regex

fn main() {
	corrupted_operation := 'xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))'
	operation_query := r'mul\((\d+),(\d+)\)'
	factor_query := r'\d+'
	mut operation_re := regex.regex_opt(operation_query) or { panic(err) }
	mut factory_re := regex.regex_opt(factor_query) or { panic(err) }
	find_all_mul := operation_re.find_all_str(corrupted_operation)
	mut total := 0
	for mul_found in find_all_mul {
		mut factor_result := 1
		factors := factory_re.find_all_str(mul_found).map(it.int())
		for factor in factors {
			factor_result *= factor
		}
		total += factor_result
	}
	println(total.str())
}
