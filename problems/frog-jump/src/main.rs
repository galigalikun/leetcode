fn main() {
    assert_eq!(Solution::can_cross(vec![0, 1, 3, 5, 6, 8, 12, 17]), true);
    assert_eq!(Solution::can_cross(vec![0, 1, 2, 3, 4, 8, 9, 11]), false);
    assert_eq!(Solution::can_cross(vec![0, 2]), false);
}

pub struct Solution {}
// https://medium.com/@gobindsingh_55492/leetcode-403-frog-jump-hard-not-much-e138b31bc209
use std::collections::HashMap;
impl Solution {
    fn helper(
        dp: &mut HashMap<(i32, i32), bool>,
        stone: HashMap<i32, bool>,
        pos: i32,
        k: i32,
        last_pos: i32,
    ) -> bool {
        if pos == last_pos {
            return true;
        }
        if k <= 0 || pos > last_pos {
            return false;
        }

        if let Some(&b) = dp.get(&(pos, k)) {
            return b;
        }
        if Some(&true) == stone.get(&pos) {
            let b = Solution::helper(dp, stone.clone(), pos + k, k, last_pos)
                || Solution::helper(dp, stone.clone(), pos + k - 1, k - 1, last_pos)
                || Solution::helper(dp, stone.clone(), pos + k + 1, k + 1, last_pos);
            dp.insert((pos, k), b);
            return b;
        }

        dp.insert((pos, k), false);
        return false;
    }
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let mut stone = HashMap::new();
        let &last_pos = stones.last().unwrap();
        let mut dp: HashMap<(i32, i32), bool> = HashMap::new();
        for s in stones {
            stone.insert(s, true);
        }
        if Some(&true) == stone.get(&1) {
            return Solution::helper(&mut dp, stone, 1, 1, last_pos);
        }
        return false;
    }
}
