fn main() {
    assert_eq!(Solution::smallest_distance_pair(vec![1, 3, 1], 1), 0);
    assert_eq!(Solution::smallest_distance_pair(vec![1, 1, 1], 2), 0);
    assert_eq!(Solution::smallest_distance_pair(vec![1, 6, 1], 3), 5);
}

struct Solution {}
impl Solution {
    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut low = 0;
        let mut high = nums[nums.len() - 1] - nums[0];

        while low < high {
            let mid = low + (high - low) / 2;
            if Self::count_pairs_with_distance_at_most(&nums, mid) >= k as i64 {
                high = mid;
            } else {
                low = mid + 1;
            }
        }

        low
    }

    fn count_pairs_with_distance_at_most(nums: &[i32], max_dist: i32) -> i64 {
        let mut count = 0_i64;
        let mut left = 0_usize;

        for right in 0..nums.len() {
            while nums[right] - nums[left] > max_dist {
                left += 1;
            }
            count += (right - left) as i64;
        }

        count
    }
}
