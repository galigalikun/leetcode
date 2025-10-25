fn main() {
    assert_eq!(Solution::stone_game_ix(vec![2, 1]), true);
    assert_eq!(Solution::stone_game_ix(vec![2]), false);
    assert_eq!(Solution::stone_game_ix(vec![5, 1, 2, 4, 3]), false);
}

struct Solution;
impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let n = stones.len();
        if n == 1 {
            return false;
        }
        let mut count = vec![0; 3];
        for &stone in &stones {
            count[(stone % 3) as usize] += 1;
        }
        if count[0] % 2 == 0 {
            return count[1] >= 1 && count[2] >= 1;
        }
        if count[1] % 2 == 1 {
            return count[2] >= 2;
        } else {
            return count[1] >= 2;
        }
    }
}
