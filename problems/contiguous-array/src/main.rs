fn main() {
    assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
    assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
    assert_eq!(Solution::find_max_length(vec![0, 1, 0, 1]), 4);
}

// https://scrapbox.io/rustacean/Contiguous_Array
struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut cur_sum = 0;
        map.insert(0, -1);
        let mut ans = 0;
        for i in 0..nums.len() {
            cur_sum += if nums[i] == 1 { 1 } else { -1 };
            if let Some(m) = map.get(&cur_sum) {
                ans = std::cmp::max(ans, i as i32 - m);
            } else {
                map.insert(cur_sum, i as i32);
            }
        }
        return ans;
    }
}
