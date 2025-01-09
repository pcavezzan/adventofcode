module diskmap

import strconv

const empty_id_value = -1

struct IndividualBlock {
	value int
}

fn (ib IndividualBlock) to_string() string {
	if ib.value == empty_id_value {
		return '.'
	}
	return ib.value.str()
}

struct Block {
	number_of_blocs int
	id              int = empty_id_value
}

fn new_file_block(id int, block int) Block {
	return Block{
		id:              id
		number_of_blocs: block
	}
}

fn new_free_space_block(block int) Block {
	return Block{
		number_of_blocs: block
	}
}

pub struct DiskMap {
	blocs []Block
}

type IndividualBlocks = []IndividualBlock

pub fn (ibs IndividualBlocks) to_string() string {
	mut to_string := ''
	for _, ib in ibs {
		to_string += ib.to_string()
	}
	return to_string
}

pub fn (dm DiskMap) individual_blocks() IndividualBlocks {
	mut individual_blocks := []IndividualBlock{}
	for block in dm.blocs {
		for i := 0; i < block.number_of_blocs; i++ {
			individual_blocks << IndividualBlock{
				value: block.id
			}
		}
	}
	return individual_blocks
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
