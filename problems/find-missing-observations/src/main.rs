fn main() {
    assert_eq!(Solution::missing_rolls(vec![3, 2, 4, 3], 4, 2), vec![6, 6]);
    assert_eq!(
        Solution::missing_rolls(vec![1, 5, 6], 3, 4),
        vec![2, 3, 2, 2]
    );
    assert_eq!(Solution::missing_rolls(vec![1, 2, 3, 4], 6, 4), vec![]);
}

struct Solution;
impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let total = rolls.len() as i32 + n * mean;
        let sum: i32 = rolls.iter().sum();
        let missing_sum = total - sum;

        if missing_sum < n || missing_sum > n * 6 {
            return vec![];
        }

        let base = missing_sum / n;
        let mut result = vec![base; n as usize];
        let remainder = missing_sum % n;

        for i in 0..remainder {
            result[i as usize] += 1;
        }

        result
    }
}
