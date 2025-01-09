module diskmap

fn test_create_diskmap_from_input() {
	input := '12345'

	disk_map := from_str_input(input)!

	assert disk_map == DiskMap{
		blocs: [
			Block{
				number_of_blocs: 1
				id:              0
			},
			Block{
				number_of_blocs: 2
				id:              -1
			},
			Block{
				number_of_blocs: 3
				id:              1
			},
			Block{
				number_of_blocs: 4
				id:              -1
			},
			Block{
				number_of_blocs: 5
				id:              2
			},
		]
	}
}

fn test_disk_map_represents_individual_blocks() {
	disk_map := from_str_input('12345')!

	individual_blocks := disk_map.individual_blocks()

	assert individual_blocks == [
		IndividualBlock{0},
		IndividualBlock{-1},
		IndividualBlock{-1},
		IndividualBlock{1},
		IndividualBlock{1},
		IndividualBlock{1},
		IndividualBlock{-1},
		IndividualBlock{-1},
		IndividualBlock{-1},
		IndividualBlock{-1},
		IndividualBlock{2},
		IndividualBlock{2},
		IndividualBlock{2},
		IndividualBlock{2},
		IndividualBlock{2},
	]
}

fn test_individual_blocks_represented_as_string() {
	individual_blocks := IndividualBlocks([
		IndividualBlock{0},
		IndividualBlock{-1},
		IndividualBlock{-1},
		IndividualBlock{1},
	])

	result := individual_blocks.to_string()

	assert result == '0..1'
}
