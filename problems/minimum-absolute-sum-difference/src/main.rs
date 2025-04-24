fn main() {
    assert_eq!(
        Solution::min_absolute_sum_diff(vec![1, 7, 5], vec![2, 3, 5]),
        3
    );
    assert_eq!(
        Solution::min_absolute_sum_diff(vec![2, 4, 6, 8, 10], vec![2, 4, 6, 8, 10]),
        10
    );
    assert_eq!(
        Solution::min_absolute_sum_diff(vec![1, 10, 4, 4, 2, 7], vec![9, 3, 5, 1, 7, 4]),
        20
    );
}

struct Solution;
impl Solution {
    pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut nums1 = nums1;
        let nums2 = nums2;
        nums1.sort();
        let mut max_diff = 0;
        let mut total_diff = 0;
        for i in 0..nums1.len() {
            let diff = (nums1[i] - nums2[i]).abs();
            total_diff += diff;
            if diff > max_diff {
                max_diff = diff;
            }
        }
        let mut min_diff = i32::MAX;
        for i in 0..nums1.len() {
            let diff = (nums1[i] - nums2[i]).abs();
            if diff == max_diff {
                let mut left = 0;
                let mut right = nums1.len() - 1;
                while left <= right {
                    let mid = (left + right) / 2;
                    if nums1[mid] < nums2[i] {
                        left = mid + 1;
                    } else {
                        if mid > 0 {
                            right = mid - 1;
                        } else {
                            break;
                        }
                    }
                }
                if left < nums1.len() {
                    min_diff = min_diff.min((nums1[left] - nums2[i]).abs());
                }
                if right > 0 {
                    min_diff = min_diff.min((nums1[right] - nums2[i]).abs());
                }
            }
        }
        let mut result = total_diff - max_diff + min_diff;
        if result < 0 {
            result = 0;
        }
        result %= 1_000_000_007;
        result
    }
}
