fn main() {
    let v1 = &mut vec![3, 2, 2, 3];
    assert_eq!(Solution::remove_element(v1, 3), 2);
    assert_eq!(v1, &mut vec![2, 2]);
    let v2 = &mut vec![0, 1, 2, 2, 3, 0, 4, 2];
    assert_eq!(Solution::remove_element(v2, 2), 5);
    assert_eq!(v2, &mut vec![0, 1, 3, 0, 4]);
}

struct Solution {}
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        while let Some(p) = nums.iter().position(|&x| x == val) {
            nums.remove(p);
        }
        return nums.len() as i32;
    }
}
