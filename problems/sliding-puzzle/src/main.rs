fn main() {
    assert_eq!(
        Solution::sliding_puzzle(vec![vec![1, 2, 3], vec![4, 0, 5]]),
        1
    );
    assert_eq!(
        Solution::sliding_puzzle(vec![vec![1, 2, 3], vec![5, 4, 0]]),
        -1
    );
    assert_eq!(
        Solution::sliding_puzzle(vec![vec![4, 1, 2], vec![5, 0, 3]]),
        5
    );
}

struct Solution {}
impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        use std::collections::{HashSet, VecDeque};

        let target = "123450";
        let neighbors: [&[usize]; 6] = [&[1, 3], &[0, 2, 4], &[1, 5], &[0, 4], &[1, 3, 5], &[2, 4]];

        let start: String = board
            .iter()
            .flat_map(|row| row.iter())
            .map(|&n| (b'0' + n as u8) as char)
            .collect();

        if start == target {
            return 0;
        }

        let mut visited: HashSet<String> = HashSet::from([start.clone()]);
        let mut queue: VecDeque<(String, i32)> = VecDeque::from([(start, 0)]);

        while let Some((state, moves)) = queue.pop_front() {
            let zero_idx = state.find('0').expect("state must contain zero tile");
            let mut chars: Vec<char> = state.chars().collect();

            for &next_idx in neighbors[zero_idx] {
                chars.swap(zero_idx, next_idx);
                let next_state: String = chars.iter().collect();

                if next_state == target {
                    return moves + 1;
                }

                if visited.insert(next_state.clone()) {
                    queue.push_back((next_state, moves + 1));
                }

                chars.swap(zero_idx, next_idx);
            }
        }

        -1
    }
}
