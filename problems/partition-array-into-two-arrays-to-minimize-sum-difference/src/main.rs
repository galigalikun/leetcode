fn main() {
    assert_eq!(Solution::minimum_difference(vec![3, 9, 7, 3]), 2);
    assert_eq!(Solution::minimum_difference(vec![-36, 36]), 72);
    assert_eq!(Solution::minimum_difference(vec![2, -1, 0, 4, -2, -9]), 0);
}

struct Solution;
impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len() / 2;
        let mut left_sums = vec![vec![]; n + 1];
        let mut right_sums = vec![vec![]; n + 1];
            for i in 0..=n {
                left_sums[i] = vec![0; 1 << n];
                right_sums[i] = vec![0; 1 << n];
            }
        for i in 0..(1 << n) {
            let mut left_count = 0;
            let mut right_count = 0;
            let mut left_sum = 0;
            let mut right_sum = 0;
            for j in 0..n {
                if (i & (1 << j)) != 0 {
                    left_count += 1;
                    left_sum += nums[j];
                    right_count += 1;
                    right_sum += nums[n + j];
                }
            }
            left_sums[left_count][i] = left_sum;
            right_sums[right_count][i] = right_sum;
        }
        for i in 0..=n {
            left_sums[i].sort_unstable();
            right_sums[i].sort_unstable();
        }
        let total_sum: i32 = nums.iter().sum();
        let mut min_diff = i32::MAX;
        for left_count in 0..=n {
            let right_count = n - left_count;
            let left_part = &left_sums[left_count];
            let right_part = &right_sums[right_count];
            for &left_sum in left_part.iter() {
                let target = total_sum / 2 - left_sum;
                let mut low = 0;
                let mut high = right_part.len() as i32 - 1;
                while low <= high {
                    let mid = (low + high) / 2;
                    let right_sum = right_part[mid as usize];
                    let current_sum = left_sum + right_sum;
                    let current_diff = (total_sum - 2 * current_sum).abs();
                    if current_diff < min_diff {
                        min_diff = current_diff;
                    }
                    if current_sum < target {
                        low = mid + 1;
                    } else {
                        high = mid - 1;
                    }
                }
            }
        }
        min_diff
    }
}
