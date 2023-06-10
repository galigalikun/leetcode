fn main() {
    assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    assert_eq!(Solution::last_stone_weight(vec![1]), 1);
}

struct Solution;
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut stones = stones;
        stones.sort();
        while stones.len() > 1 {
            let len = stones.len();
            let diff = stones[len - 1] - stones[len - 2];
            stones.pop();
            stones.pop();
            if diff > 0 {
                let mut i = 0;
                while i < stones.len() && stones[i] < diff {
                    i += 1;
                }
                stones.insert(i, diff);
            }
        }
        if stones.len() > 0 {
            return stones[0];
        }
        return 0;
    }
}
