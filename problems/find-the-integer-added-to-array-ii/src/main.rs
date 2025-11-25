fn main() {
    assert_eq!(
        Solution::minimum_added_integer(vec![4, 20, 16, 12, 8], vec![14, 18, 10]),
        -2
    );
    assert_eq!(
        Solution::minimum_added_integer(vec![3, 5, 5, 3], vec![7, 7]),
        2
    );
}

struct Solution;
impl Solution {
    pub fn minimum_added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let set1: std::collections::HashSet<i32> = nums1.into_iter().collect();
        let set2: std::collections::HashSet<i32> = nums2.into_iter().collect();

        for i in 1.. {
            if !set1.contains(&i) && !set2.contains(&i) {
                return i;
            }
        }
        -1
    }
}
