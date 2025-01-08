module diskmap

import strconv

enum BlockType {
	file
	free_space
}

const empty_id_value = -1

struct Block {
	bloc_type       BlockType
	number_of_blocs int
	id              int = empty_id_value
}

fn new_file_block(id int, block int) Block {
	return Block{
		id:              id
		bloc_type:       BlockType.file
		number_of_blocs: block
	}
}

fn new_free_space_block(block int) Block {
	return Block{
		bloc_type:       BlockType.free_space
		number_of_blocs: block
	}
}

pub struct DiskMap {
	blocs []Block
}

pub fn from_str_input(input string) !DiskMap {
	mut next_id := 0
	block_sequences := input.runes().map(strconv.atoi(it.str())!)
	mut blocks := []Block{}
	for i, block in block_sequences {
		if i % 2 == 0 {
			blocks << new_file_block(next_id, block)
			next_id++
		} else {
			blocks << new_free_space_block(block)
		}
	}
	return DiskMap{
		blocs: blocks
	}
}
