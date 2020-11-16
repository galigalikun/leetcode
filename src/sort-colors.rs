fn main() {
    let n1 = &mut vec![2, 0, 2, 1, 1, 0];
    Solution::sort_colors(n1);
    assert_eq!(n1, &mut vec![0, 0, 1, 1, 2, 2]);
    let n2 = &mut vec![2, 0, 1];
    Solution::sort_colors(n2);
    assert_eq!(n2, &mut vec![0, 1, 2]);
    let n3 = &mut vec![0];
    Solution::sort_colors(n3);
    assert_eq!(n3, &mut [0]);
}

pub struct Solution {}
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        nums.sort();
    }
}
