fn main() {
    assert_eq!(Solution::subarrays_with_k_distinct(vec![1,2,1,2,3], 2), 7);
    assert_eq!(Solution::subarrays_with_k_distinct(vec![1,2,1,3,4], 3), 3);
}

struct Solution;
impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let mut left = 0;
        let mut right = 0;
        let mut map = std::collections::HashMap::new();
        while right < nums.len() {
            let num = nums[right];
            let mut count = *map.entry(num).or_insert(0) + 1;
            if count == 1 {
                count += 1;
            }
            while count > k {
                let num = nums[left];
                count = *map.entry(num).or_insert(0) - 1;
                if count == 0 {
                    count -= 1;
                }
                left += 1;
            }
            result += right - left + 1;
            right += 1;
        }
        return result as i32;
    }
}
