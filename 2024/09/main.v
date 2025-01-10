module main

import diskfragmenter

fn main() {
	mut disk := diskfragmenter.from_map('2333133121414131402')!
	println(disk.to_string())
	for {
		disk.compact()
		println(disk.to_string())
		if !disk.needs_compaction() {
			break
		}
	}
}
