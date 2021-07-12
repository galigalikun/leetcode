fn main() {
    assert_eq!(
        Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]),
        vec![2, 2]
    );

    assert_eq!(
        Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
        vec![4, 9]
    );

    /*
        [1,2,2,1]
    [2]
        */
    assert_eq!(Solution::intersect(vec![1, 2, 2, 1], vec![2]), vec![2]);
}

pub struct Solution {}
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut rv = nums2;
        let mut result = vec![];
        for n in nums1 {
            if let Some(p) = rv.iter().position(|&x| x == n) {
                rv.remove(p);
                result.push(n);
            }
        }
        return result;
    }
}
