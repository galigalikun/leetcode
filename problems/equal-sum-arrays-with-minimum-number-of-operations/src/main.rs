fn main() {
    assert_eq!(
        Solution::min_operations(vec![1, 2, 3, 4, 5, 6], vec![1, 1, 2, 2, 2, 2]),
        3
    );
    assert_eq!(
        Solution::min_operations(vec![1, 1, 1, 1, 1, 1, 1], vec![6]),
        -1
    );
    assert_eq!(Solution::min_operations(vec![6, 6], vec![1]), 3);
}

struct Solution;
impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let sum1 = nums1.iter().sum::<i32>();
        let sum2 = nums2.iter().sum::<i32>();
        if sum1 == sum2 {
            return 0;
        }
        if sum1 > sum2 {
            return Solution::min_operations(nums2, nums1);
        }
        let mut diff = sum2 - sum1;
        let mut freq1 = [0; 6];
        let mut freq2 = [0; 6];
        for &num in &nums1 {
            freq1[(num - 1) as usize] += 1;
        }
        for &num in &nums2 {
            freq2[(num - 1) as usize] += 1;
        }
        let mut ans = 0;
        for i in 0..5 {
            let diff1 = 5 - i;
            let diff2 = i;
            let freq = freq1[i] + freq2[5 - i];
            let take1 = diff1 / 5 + if diff1 % 5 > 0 { 1 } else { 0 };
            let take2 = diff2 / 5 + if diff2 % 5 > 0 { 1 } else { 0 };
            if diff <= freq {
                return (ans + take1 + take2) as i32;
            }
            diff -= freq;
            ans += freq as usize;
        }
        ans += diff as usize / 5 + if diff % 5 > 0 { 1 } else { 0 };
        if ans > nums1.len() + nums2.len() {
            return -1;
        }
        ans as i32
    }
}
