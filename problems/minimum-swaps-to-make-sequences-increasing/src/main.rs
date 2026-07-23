fn main() {
    assert_eq!(Solution::min_swap(vec![1, 3, 5, 4], vec![1, 2, 3, 7]), 1);
    assert_eq!(
        Solution::min_swap(vec![0, 3, 5, 8, 9], vec![2, 1, 4, 6, 9]),
        1
    );
}

struct Solution {}
impl Solution {
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        if n <= 1 {
            return 0;
        }

        let mut keep: i32 = 0;
        let mut swap: i32 = 1;

        for i in 1..n {
            let mut next_keep = i32::MAX;
            let mut next_swap = i32::MAX;

            if nums1[i] > nums1[i - 1] && nums2[i] > nums2[i - 1] {
                next_keep = next_keep.min(keep);
                next_swap = next_swap.min(swap.saturating_add(1));
            }

            if nums1[i] > nums2[i - 1] && nums2[i] > nums1[i - 1] {
                next_keep = next_keep.min(swap);
                next_swap = next_swap.min(keep.saturating_add(1));
            }

            keep = next_keep;
            swap = next_swap;
        }

        keep.min(swap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_case_1() {
        assert_eq!(Solution::min_swap(vec![1, 3, 5, 4], vec![1, 2, 3, 7]), 1);
    }

    #[test]
    fn sample_case_2() {
        assert_eq!(
            Solution::min_swap(vec![0, 3, 5, 8, 9], vec![2, 1, 4, 6, 9]),
            1
        );
    }

    #[test]
    fn already_increasing_no_swap() {
        assert_eq!(Solution::min_swap(vec![1, 2, 3], vec![4, 5, 6]), 0);
    }
}
