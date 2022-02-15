fn main() {
    assert_eq!(Solution::maximum_product(vec![1, 2, 3]), 6);
    assert_eq!(Solution::maximum_product(vec![1, 2, 3, 4]), 24);
    assert_eq!(Solution::maximum_product(vec![-1, -2, -3]), -6);
    assert_eq!(
        Solution::maximum_product(vec![-100, -98, -1, 2, 3, 4]),
        39200
    );
    assert_eq!(Solution::maximum_product(vec![-100, -2, -3, 1]), 300);
}

struct Solution {}
impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        if nums.len() == 3 {
            return nums.iter().fold(1, |a, b| a * b);
        }
        let mut positive = nums.iter().filter(|&a| a >= &0).collect::<Vec<_>>();
        let mut negative = nums.iter().filter(|&a| a < &0).collect::<Vec<_>>();
        positive.sort();
        negative.sort();
        let ans = if positive.len() > 2 {
            positive[positive.len() - 1]
                * positive[positive.len() - 2]
                * positive[positive.len() - 3]
        } else if positive.len() == 1 {
            negative[0] * negative[1] * positive[positive.len() - 1]
        } else {
            negative[negative.len() - 1]
                * negative[negative.len() - 2]
                * negative[negative.len() - 3]
        };
        if negative.len() > 1 && positive.len() > 0 {
            return std::cmp::max(
                ans,
                negative[0] * negative[1] * positive[positive.len() - 1],
            );
        }

        return ans;
    }
}
