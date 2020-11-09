fn main() {
    assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
}

pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut num = nums[0] + nums[1] + nums[2];
        if nums.len() == 3 {
            return num;
        }
        for x in 0..nums.len()-2 {
            for y in 1..nums.len()-1 {
                for z in 1..nums.len() {
                    if x == y {
                        continue;
                    }
                    if y == z {
                        continue;
                    }
                    if z == x {
                        continue;
                    }
                    let sum = nums[x] + nums[y] + nums[z];
                    if target == sum {
                        return target;
                    }
                    if (target - num).abs() > (target - sum).abs() {
                        num = sum;
                    }
                }
            }
        }
        return num;
    }
}
