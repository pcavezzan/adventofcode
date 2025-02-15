module topographic

fn test_new_map_from_puzzle() {
	topographic_map := new_map_from_puzzle('0123\n1234\n8765\n9876')

	assert topographic_map == Map{
		positions: [
			[Position{
				height: 0
			}, Position{
				height: 1
			}, Position{
				height: 2
			}, Position{
				height: 3
			}],
			[Position{
				height: 1
			}, Position{
				height: 2
			}, Position{
				height: 3
			}, Position{
				height: 4
			}],
			[Position{
				height: 8
			}, Position{
				height: 7
			}, Position{
				height: 6
			}, Position{
				height: 5
			}],
			[Position{
				height: 9
			}, Position{
				height: 8
			}, Position{
				height: 7
			}, Position{
				height: 6
			}],
		]
	}
}
