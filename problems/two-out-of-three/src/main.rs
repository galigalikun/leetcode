fn main() {
    assert_eq!(
        Solution::two_out_of_three(vec![1, 1, 3, 2], vec![2, 3], vec![3]),
        vec![3, 2]
    );
    assert_eq!(
        Solution::two_out_of_three(vec![3, 1], vec![2, 3], vec![1, 2]),
        vec![2, 3, 1]
    );
    assert_eq!(
        Solution::two_out_of_three(vec![1, 2, 2], vec![4, 3, 3], vec![5]),
        vec![]
    );
}

struct Solution;
impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut counts: std::collections::HashMap<i32, u8> = std::collections::HashMap::new();
        for &num in nums1.iter().collect::<std::collections::HashSet<_>>().iter() {
            *counts.entry(*num).or_insert(0) |= 1;
        }
        for &num in nums2.iter().collect::<std::collections::HashSet<_>>().iter() {
            *counts.entry(*num).or_insert(0) |= 2;
        }
        for &num in nums3.iter().collect::<std::collections::HashSet<_>>().iter() {
            *counts.entry(*num).or_insert(0) |= 4;
        }
        counts.into_iter()
            .filter(|&(_, v)| v.count_ones() > 1)
            .map(|(k, _)| k)
            .collect()
    }
}
