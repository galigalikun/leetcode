fn main() {
    assert_eq!(Solution::triangle_number(vec![2, 2, 3, 4]), 3);
    assert_eq!(Solution::triangle_number(vec![4, 2, 3, 4]), 4);
    assert_eq!(Solution::triangle_number(vec![2, 3, 4]), 1);
    assert_eq!(Solution::triangle_number(vec![1, 3, 4]), 0);
    assert_eq!(Solution::triangle_number(vec![1,2,3,4,5,6]), 7);
}

struct Solution {}
impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        // a + b > c
        // b + c > a
        // c + a > b
        return match nums.len() {
            0 | 1 | 2 => 0,
            3 => {
                let a = nums[0];
                let b = nums[1];
                let c = nums[2];
                if a + b > c && b + c > a && c + a > b {
                    1
                } else {
                    0
                }
            }
            _ => {
                let mut ans = 0;
                for i in 0..nums.len() {
                    let a = nums[i];
                    let b = nums[(i + 1) % nums.len()];
                    let c = nums[(i + 2) % nums.len()];
                    if a + b > c && b + c > a && c + a > b {
                        ans += 1;
                    }
                }
                ans
            }
        };
    }
}
