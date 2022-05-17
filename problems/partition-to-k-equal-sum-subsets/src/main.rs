fn main() {
    assert_eq!(
        Solution::can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4),
        true
    );
    assert_eq!(
        Solution::can_partition_k_subsets(vec![1, 2, 3, 4], 3),
        false
    );
    assert_eq!(
        Solution::can_partition_k_subsets(vec![2, 2, 2, 2, 3, 4, 5], 4),
        false
    );
}

struct Solution {}
impl Solution {

    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % k != 0 {
            return false;
        }
        // ??
        return false;
    }
}
