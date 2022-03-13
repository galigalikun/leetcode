fn main() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
        2.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
        2.5
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![0, 0], vec![0, 0]),
        0.0
    );
    assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![2], vec![]), 2.0);
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 7]),
        2.5
    );
    // 1, 2, 3, 7 => 2 3
}

struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut a = Vec::new();
        a.extend(nums1);
        a.extend(nums2);
        a.sort();
        if a.len() % 2 == 1 {
            return a[a.len() / 2] as f64;
        }
        return ((a[a.len() / 2 - 1] + a[a.len() / 2]) as f64) / 2.0;
    }
}
