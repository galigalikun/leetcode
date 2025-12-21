fn main() {
    assert_eq!(
        Solution::has_increasing_subarrays(vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1], 3),
        true
    );
    assert_eq!(
        Solution::has_increasing_subarrays(vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7], 5),
        false
    );
}

struct Solution;
impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        if n < k as usize {
            return false;
        }

        for i in 0..=n - k as usize {
            let mut increasing = true;
            for j in i..i + k as usize - 1 {
                if nums[j] >= nums[j + 1] {
                    increasing = false;
                    break;
                }
            }
            if increasing {
                return true;
            }
        }

        false
    }
}
