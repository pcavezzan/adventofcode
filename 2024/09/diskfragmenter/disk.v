module diskfragmenter

import strconv

const empty_id_value = -1

struct Block {
	value int
}

fn (b Block) is_different_from(other Block) bool {
	return b.value != other.value
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

pub struct Disk {
mut:
	blocs Blocks
}

pub fn (d Disk) to_string() string {
	return d.blocs.to_string()
}

pub fn (mut d Disk) compact() Disk {
	last_index_file_block, first_index_free_block := d.find_first_space_and_last_file_blocks()
	last_file_block := d.blocs[last_index_file_block]
	first_free_block := d.blocs[first_index_free_block]
	d.blocs[first_index_free_block] = last_file_block
	d.blocs[last_index_file_block] = first_free_block
	return d
}

fn (d Disk) find_first_space_and_last_file_blocks() (int, int) {
	out_of_index := -1
	mut first_index_free_block := out_of_index
	mut last_index_file_block := out_of_index
	for index, block in d.blocs {
		if block.value == -1 {
			if first_index_free_block == out_of_index {
				first_index_free_block = index
			}
		} else {
			last_index_file_block = index
		}
	}

	return last_index_file_block, first_index_free_block
}

pub fn (d Disk) needs_compaction() bool {
	last_index_file_block, first_index_free_block := d.find_first_space_and_last_file_blocks()
	if first_index_free_block > last_index_file_block {
		return false
	}

	return true
}

type Blocks = []Block

pub fn (ibs Blocks) to_string() string {
	mut to_string := ''
	for _, ib in ibs {
		to_string += ib.to_string()
	}
	return to_string
}

pub fn from_map(input string) !Disk {
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
	return Disk{
		blocs: BlocsDenseFormat(denses_format).to_individual_blocks()
	}
}
