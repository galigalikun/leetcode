fn main() {
    assert_eq!(Solution::minimum_distance(vec![1, 2, 1, 1, 3]), 6);
    assert_eq!(Solution::minimum_distance(vec![1, 1, 2, 3, 2, 1, 2]), 8);
    assert_eq!(Solution::minimum_distance(vec![1]), -1);
}

struct Solution;
impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        let mut ans = i32::MAX;
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&num) {
                ans = ans.min((i - j) as i32);
            }
            map.insert(num, i);
        }
        if ans == i32::MAX { -1 } else { ans }
    }
}
