module main

import diskmap

fn main() {
	disk_map := diskmap.from_str_input('2333133121414131402')!
	println(disk_map.to_string())
}
