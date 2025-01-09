module main

import diskmap

fn main() {
	disk_map := diskmap.from_str_input('2333133121414131402')!

	blocks := disk_map.individual_blocks()

	println(blocks.to_string())
}
