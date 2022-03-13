fn main() {
    assert_eq!(
        Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
        vec![[1, 2], [1, 4], [1, 6]]
    );
    assert_eq!(
        Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2),
        vec![[1, 1], [1, 1]]
    );
    assert_eq!(
        Solution::k_smallest_pairs(vec![1, 2], vec![3], 3),
        vec![[1, 3], [2, 3]]
    );
}

struct Solution {}
// https://www.geeksforgeeks.org/find-k-pairs-smallest-sums-two-arrays/
impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut idx = vec![0; nums1.len()];
        let mut kk = k;
        while kk > 0 {
            let mut min_sum = std::i32::MAX;
            let mut min_idx = 0;
            for i in 0..nums1.len() {
                if idx[i] < nums2.len() && nums1[i] + nums2[idx[i]] < min_sum {
                    min_idx = i;
                    min_sum = nums1[i] + nums2[idx[i]];
                }
            }
            if idx[min_idx] < nums2.len() {
                result.push(vec![nums1[min_idx], nums2[idx[min_idx]]]);
            }

            idx[min_idx] += 1;
            kk -= 1;
        }
        return result;
    }
}
