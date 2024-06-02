fn main() {
    assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
    assert_eq!(Solution::max_product(vec![1, 5, 4, 5]), 16);
    assert_eq!(Solution::max_product(vec![3, 7]), 12);
}

struct Solution;
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max1 = 0;
        let mut max2 = 0;
        for n in nums {
            if n > max1 {
                max2 = max1;
                max1 = n;
            } else if n > max2 {
                max2 = n;
            }
        }
        return (max1 - 1) * (max2 - 1);
    }
}
