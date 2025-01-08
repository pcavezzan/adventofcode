module diskmap

fn test_create_diskmap_from_input() {
	input := '12345'

	disk_map := from_str_input(input)!

	assert disk_map == DiskMap{
		blocs: [
			Block{
				bloc_type:       .file
				number_of_blocs: 1
				id:              0
			},
			Block{
				bloc_type:       .free_space
				number_of_blocs: 2
				id:              -1
			},
			Block{
				bloc_type:       .file
				number_of_blocs: 3
				id:              1
			},
			Block{
				bloc_type:       .free_space
				number_of_blocs: 4
				id:              -1
			},
			Block{
				bloc_type:       .file
				number_of_blocs: 5
				id:              2
			},
		]
	}
}
