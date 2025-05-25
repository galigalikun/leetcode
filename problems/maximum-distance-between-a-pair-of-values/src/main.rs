fn main() {
    assert_eq!(
        Solution::max_distance(vec![55, 30, 5, 4, 2], vec![100, 20, 10, 10, 5]),
        2
    );
    assert_eq!(Solution::max_distance(vec![2, 2, 2], vec![10, 10, 1]), 1);
    assert_eq!(
        Solution::max_distance(vec![30, 29, 19, 5], vec![25, 25, 25, 25, 25]),
        2
    );
}

struct Solution;
impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        return nums1
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                nums2
                    .iter()
                    .enumerate()
                    .filter(|&(_, &y)| y >= x)
                    .map(|(j, _)| j as i32 - i as i32)
                    .max()
                    .unwrap_or(-1)
            })
            .max()
            .unwrap_or(-1);
    }
}
