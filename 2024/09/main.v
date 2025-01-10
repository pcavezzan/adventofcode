module main

import diskfragmenter

fn main() {
	mut disk := diskfragmenter.from_map('2333133121414131402')!
	for {
		disk.compact()
		if !disk.needs_compaction() {
			break
		}
	}
	println('disk checksum: ${disk.checksum()}')
}
