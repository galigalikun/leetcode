fn main() {
    assert_eq!(Solution::tuple_same_product(vec![2, 3, 4, 6]), 8);
    assert_eq!(Solution::tuple_same_product(vec![1, 2, 4, 5, 10]), 16);
}

struct Solution;
impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let mut res = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let product = nums[i] * nums[j];
                if let Some(&count) = map.get(&product) {
                    res += count;
                }
                map.entry(product).and_modify(|e| *e += 1).or_insert(1);
            }
        }
        res * 8
    }
}
