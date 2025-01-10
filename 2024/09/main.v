module main

import diskmap

fn main() {
	mut disk_map := diskmap.from_str_input('2333133121414131402')!
	println(disk_map.to_string())
	for {
		disk_map.compact()
		println(disk_map.to_string())
		if !disk_map.needs_compaction() {
			break
		}
	}
}
