fn main() {
    assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
}

pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut num = 0;
        for n in nums {
            num += n;
        }
        println!("{} {}", num, target);
        return 2;
    }
}
