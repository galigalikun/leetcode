use std::collections::VecDeque;

fn main() {
    assert_eq!(Solution::deck_revealed_increasing(vec![17,13,11,2,3,5,7]), vec![2,13,3,11,5,17,7]);
    assert_eq!(Solution::deck_revealed_increasing(vec![1,1000]), vec![1,1000]);
}

struct Solution;
impl Solution {
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        let mut sorted_deck = deck;
        sorted_deck.sort_unstable();

        let n = sorted_deck.len();
        let mut result = vec![0; n];
        let mut positions: VecDeque<usize> = (0..n).collect();

        for card in sorted_deck {
            let idx = positions.pop_front().expect("positions should not be empty");
            result[idx] = card;

            if let Some(next_idx) = positions.pop_front() {
                positions.push_back(next_idx);
            }
        }

        result
    }
}
