fn main() {
    assert_eq!(
        Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
        vec![-1, 3, -1]
    );
    assert_eq!(
        Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
        vec![3, -1]
    );
    assert_eq!(
        Solution::next_greater_element(vec![1, 3, 5, 2, 4], vec![6, 5, 4, 3, 2, 1, 7]),
        vec![7, 7, 7, 7, 7]
    );
}

struct Solution {}
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        for n1 in nums1 {
            let mut pos = None;
            let mut b = false;
            for n2 in nums2.clone() {
                if b {
                    if n2 > n1 {
                        pos = Some(n2);
                        break;
                    }
                } else if n1 == n2 {
                    b = true;
                }
            }
            result.push(if let Some(p) = pos { p } else { -1 });
        }
        return result;
    }
}
