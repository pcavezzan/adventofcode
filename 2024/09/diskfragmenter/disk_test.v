module diskfragmenter

fn test_create_disk_from_input() {
	input := '12345'

	disk_map := from_map(input)!

	assert disk_map == Disk{
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

fn test_should_compact_disk_by_moving_blocks_one_at_a_time_from_end_to_the_leftmost_free_space_block() {
	mut disk_map := from_map('12345')!

	new_disk_map := disk_map.compact()

	assert new_disk_map == Disk{
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
	disk            Disk
}

fn test_should_indicate_when_disk_needs_compaction() {
	tests := [
		ShouldIndicateWhenDiskNeedsCompactionEntryTest{
			name:            'should return true when space exists before file block'
			expected_result: true
			disk:            Disk{
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
			disk:            Disk{
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
