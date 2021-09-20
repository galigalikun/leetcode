fn main() {
    assert_eq!(
        Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]),
        7
    );

    assert_eq!(
        Solution::number_of_arithmetic_slices(vec![7, 7, 7, 7, 7]),
        16
    );

    assert_eq!(
        Solution::number_of_arithmetic_slices(vec![0, 2000000000, -294967296]),
        0
    );
}

pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut vec: Vec<HashMap<i64, i32>> = vec![HashMap::new(); nums.len()];
        for i in 0..nums.len() {
            vec[i] = HashMap::new();
            for j in (0..i).rev() {
                let diff = nums[i] as i64 - nums[j] as i64;
                let c1 = vec[i].get(&diff).unwrap_or(&0);
                let c2 = vec[j].get(&diff).unwrap_or(&0);
                let v = c1 + c2 + 1;
                result += c2;
                vec[i].insert(diff, v);
            }
        }
        return result;
    }
}
