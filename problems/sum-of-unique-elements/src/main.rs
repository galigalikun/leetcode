fn main() {
    assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 2]), 4);
    assert_eq!(Solution::sum_of_unique(vec![1, 1, 1, 1, 1]), 0);
    assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 4, 5]), 15);
}

struct Solution;
impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut map = std::collections::HashMap::new();
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }
        for (key, value) in map {
            if value == 1 {
                sum += key;
            }
        }
        return sum;
    }
}
