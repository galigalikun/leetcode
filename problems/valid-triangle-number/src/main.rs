fn main() {
    assert_eq!(Solution::triangle_number(vec![2, 2, 3, 4]), 3);
    assert_eq!(Solution::triangle_number(vec![4, 2, 3, 4]), 4);
    assert_eq!(Solution::triangle_number(vec![2, 3, 4]), 1);
    assert_eq!(Solution::triangle_number(vec![1, 3, 4]), 0);
    assert_eq!(Solution::triangle_number(vec![1,2,3,4,5,6]), 7);
}

// https://www.geeksforgeeks.org/find-number-of-triangles-possible/
struct Solution {}
impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let mut n = nums;
        n.sort();
        let mut ans = 0;
        for i in (1..n.len()).rev() {
            let mut l = 0;
            let mut r = i - 1;
            while l < r {
                if n[l] + n[r] > n[i] {
                    ans += r - l;
                    r -= 1;
                } else {
                    l += 1;
                }
            }
        }
        return ans as i32;
    }
}
