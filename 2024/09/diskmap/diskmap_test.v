module diskmap

fn test_create_diskmap_from_input() {
	input := '12345'

	disk_map := from_str_input(input)!

	assert disk_map == DiskMap{
		blocs: [
			Block{0},
			Block{-1},
			Block{-1},
			Block{1},
			Block{1},
			Block{1},
			Block{-1},
			Block{-1},
			Block{-1},
			Block{-1},
			Block{2},
			Block{2},
			Block{2},
			Block{2},
			Block{2},
		]
	}
}

fn test_blocks_represented_as_string() {
	blocks := Blocks([
		Block{0},
		Block{-1},
		Block{-1},
		Block{1},
	])

	result := blocks.to_string()

	assert result == '0..1'
}

fn test_should_compact_disk_map_move_blocks_one_at_a_time_from_end_to_the_leftmost_free_space_block() {
	mut disk_map := from_str_input('12345')!

	new_disk_map := disk_map.compact()

	assert new_disk_map == DiskMap{
		blocs: Blocks([
			Block{0},
			Block{2},
			Block{-1},
			Block{1},
			Block{1},
			Block{1},
			Block{-1},
			Block{-1},
			Block{-1},
			Block{-1},
			Block{2},
			Block{2},
			Block{2},
			Block{2},
			Block{-1},
		])
	}
}

struct ShouldIndicateWhenDiskNeedsCompactionEntryTest {
	name            string
	expected_result bool
	disk            DiskMap
}

fn test_should_indicate_when_disk_needs_compaction() {
	tests := [
		ShouldIndicateWhenDiskNeedsCompactionEntryTest{
			name:            'should return true when space exists before file block'
			expected_result: true
			disk:            DiskMap{
				blocs: [
					Block{0},
					Block{-1},
					Block{-1},
					Block{1},
				]
			}
		},
		ShouldIndicateWhenDiskNeedsCompactionEntryTest{
			name:            'should return false when all file block are before space block'
			expected_result: false
			disk:            DiskMap{
				blocs: [
					Block{0},
					Block{1},
					Block{-1},
					Block{-1},
				]
			}
		},
	]

	for test in tests {
		result := test.disk.needs_compaction()

		expected_result := test.expected_result
		assert result == expected_result, '${test.name} - Expected: ${expected_result} Got: ${result} '
	}
}
