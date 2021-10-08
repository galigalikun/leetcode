fn main() {
    assert_eq!(Solution::can_i_win(10, 11), false);
    assert_eq!(Solution::can_i_win(10, 0), true);
    assert_eq!(Solution::can_i_win(10, 1), true);
    assert_eq!(Solution::can_i_win(4, 6), true);
    assert_eq!(Solution::can_i_win(20, 189), false);
}

pub struct Solution {}
// https://aaronice.gitbook.io/lintcode/minimax/can-i-win
use std::collections::HashMap;
impl Solution {
    fn helper(map: &mut HashMap<i32, bool>, used: &mut Vec<bool>, desired_total: i32) -> bool {
        if desired_total <= 0 {
            return false;
        }
        let mut key = 0;
        for b in used.clone() {
            key = key << 1;
            if b {
                key = key | 1;
            }
        }
        if None == map.get(&key) {
            for i in 1..used.len() {
                if !used[i] {
                    used[i] = true;
                    if !Solution::helper(map, used, desired_total - i as i32) {
                        if let Some(m) = map.get_mut(&key) {
                            *m = true;
                        }
                        used[i] = false;
                        return true;
                    }
                    used[i] = false;
                }
            }
            map.insert(key, false);
        }
        return *map.get(&key).unwrap_or(&false);
    }
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        let sum = max_choosable_integer * (max_choosable_integer + 1) / 2;
        if sum < desired_total {
            return false;
        }
        if desired_total <= max_choosable_integer {
            return true;
        }
        let mut map: HashMap<i32, bool> = HashMap::new();
        let mut used = vec![false; max_choosable_integer as usize + 1];
        return Solution::helper(&mut map, &mut used, desired_total);
    }
}
