module topographic

struct Position {
	x      int
	y      int
	height int
}

struct Map {
	positions [][]Position
}

pub fn new_map_from_puzzle(puzzle string) &Map {
	lines := puzzle.split_into_lines()
	mut positions := [][]Position{len: 0, cap: lines.len}
	for row_number, line in lines {
		runes := line.runes()
		mut row := []Position{len: 0, cap: runes.len}
		for column, c in runes {
			height_value := c.str().int()
			row << Position{
				height: height_value
				x:      column
				y:      row_number
			}
		}
		positions << row
	}
	return &Map{
		positions: positions
	}
}

struct TrailHead {
	visited []Position
}

fn (m Map) find_hiking_trails() TrailHead {
	mut next_pos := Position{}
	mut start := false
	for row, row_positions in m.positions {
		for column, pos in row_positions {
			//
			if pos.height == 1 {
				next_pos = pos
				start = true
				break
			}
		}
	}

	if !start {
		return TrailHead{}
	}

	counter := 0
	for {
		next_height := next_pos.height

		if counter == 10 {
			break
		}
	}

	return TrailHead{}
}
