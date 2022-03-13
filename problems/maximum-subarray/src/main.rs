fn main() {
    assert_eq!(
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        6
    );
    assert_eq!(Solution::max_sub_array(vec![1]), 1);
    assert_eq!(Solution::max_sub_array(vec![0]), 0);
    assert_eq!(Solution::max_sub_array(vec![-1]), -1);
    assert_eq!(Solution::max_sub_array(vec![-2147483647]), -2147483647);
}

struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct MaxSub {
    pub max: i32,
    pub current: i32,
}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut aa = Vec::new();
        aa.push(MaxSub {
            max: nums[0],
            current: nums[0],
        });
        for i in 1..nums.len() {
            aa.push(MaxSub {
                max: nums[i],
                current: nums[i],
            });
            for j in 1..=i {
                aa[i - j].current = aa[i - j].current + nums[i];
                if aa[i - j].current > aa[i - j].max {
                    aa[i - j].max = aa[i - j].current;
                }
            }
        }
        let mut result = aa[0].max;
        for a in aa {
            if a.max > result {
                result = a.max;
            }
        }
        return result;
    }
}
