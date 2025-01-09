module diskmap

import strconv

const empty_id_value = -1

struct Block {
	value int
}

fn (ib Block) to_string() string {
	if ib.value == empty_id_value {
		return '.'
	}
	return ib.value.str()
}

struct DenseFormat {
	number_of_blocs int
	id              int = empty_id_value
}

fn new_file_block_format(id int, block int) DenseFormat {
	return DenseFormat{
		id:              id
		number_of_blocs: block
	}
}

fn new_free_space_block_format(block int) DenseFormat {
	return DenseFormat{
		number_of_blocs: block
	}
}

type BlocsDenseFormat = []DenseFormat

fn (blocs BlocsDenseFormat) to_individual_blocks() Blocks {
	mut individual_blocks := []Block{}
	for block in blocs {
		for i := 0; i < block.number_of_blocs; i++ {
			individual_blocks << Block{
				value: block.id
			}
		}
	}
	return individual_blocks
}

pub struct DiskMap {
	blocs Blocks
}

pub fn (dm DiskMap) to_string() string {
	return dm.blocs.to_string()
}

type Blocks = []Block

pub fn (ibs Blocks) to_string() string {
	mut to_string := ''
	for _, ib in ibs {
		to_string += ib.to_string()
	}
	return to_string
}

pub fn from_str_input(input string) !DiskMap {
	mut next_id := 0
	block_sequences := input.runes().map(strconv.atoi(it.str())!)
	mut denses_format := []DenseFormat{}
	for i, block in block_sequences {
		if i % 2 == 0 {
			denses_format << new_file_block_format(next_id, block)
			next_id++
		} else {
			denses_format << new_free_space_block_format(block)
		}
	}
	return DiskMap{
		blocs: BlocsDenseFormat(denses_format).to_individual_blocks()
	}
}
