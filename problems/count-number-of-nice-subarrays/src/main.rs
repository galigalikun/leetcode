fn main() {
    assert_eq!(Solution::number_of_subarrays(vec![1, 1, 2, 1, 1], 3), 2);
    assert_eq!(Solution::number_of_subarrays(vec![2, 4, 6], 1), 0);
    assert_eq!(
        Solution::number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2),
        16
    );
}

struct Solution;
impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut odd = vec![];
        let mut count = 0;
        for i in 0..nums.len() {
            if nums[i] % 2 == 1 {
                odd.push(i);
            }
        }
        if odd.len() < k as usize {
            return 0;
        }
        for i in 0..odd.len() - k as usize + 1 {
            let left_count = if i == 0 {
                odd[i] + 1
            } else {
                odd[i] - odd[i - 1]
            };
            let right_count = if i + k as usize == odd.len() {
                nums.len() - odd[i + k as usize - 1]
            } else {
                odd[i + k as usize] - odd[i + k as usize - 1]
            };
            count += left_count * right_count;
        }
        return count as i32;
    }
}
