module diskmap

fn test_create_diskmap_from_input() {
	input := '12345'

	disk_map := from_str_input(input)!

	assert disk_map == DiskMap{
		blocs: [
			Block{0},
			Block{-1},
			Block{-1},
			Block{1},
			Block{1},
			Block{1},
			Block{-1},
			Block{-1},
			Block{-1},
			Block{-1},
			Block{2},
			Block{2},
			Block{2},
			Block{2},
			Block{2},
		]
	}
}

fn test_blocks_represented_as_string() {
	blocks := Blocks([
		Block{0},
		Block{-1},
		Block{-1},
		Block{1},
	])

	result := blocks.to_string()

	assert result == '0..1'
}
