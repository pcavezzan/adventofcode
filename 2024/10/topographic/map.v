module topographic

struct Position {
	height int
}

struct Map {
	positions [][]Position
}

pub fn new_map_from_puzzle(puzzle string) &Map {
	lines := puzzle.split_into_lines()
	mut positions := [][]Position{len: 0, cap: lines.len}
	for line in lines {
		runes := line.runes()
		mut row := []Position{len: 0, cap: runes.len}
		for c in runes {
			height_value := c.str().int()
			row << Position{
				height: height_value
			}
		}
		positions << row
	}
	return &Map{
		positions: positions
	}
}

struct TrailHead {
	visited []int
}

fn (m Map) find_hiking_trails() TrailHead {
	return TrailHead{}
}
