fn main() {
    assert_eq!(
        Solution::contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0),
        true
    );
    assert_eq!(
        Solution::contains_nearby_almost_duplicate(vec![1, 0, 1, 1], 1, 2),
        true
    );
    assert_eq!(
        Solution::contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3),
        false
    );
    assert_eq!(
        Solution::contains_nearby_almost_duplicate(vec![1, 2, 2, 3, 4, 5], 3, 0),
        true
    );
    assert_eq!(
        Solution::contains_nearby_almost_duplicate(vec![-2147483648, 2147483647], 1, 1),
        false
    );
}

struct Solution {}
impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i != j {
                    if (nums[i] as i64 - nums[j] as i64).abs() <= t as i64 {
                        if (i as i32 - j as i32).abs() <= k {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }
}
