fn main() {
    assert_eq!(Solution::count_nice_pairs(vec![42, 11, 1, 97]), 2);
    assert_eq!(Solution::count_nice_pairs(vec![13, 10, 35, 24, 76]), 4);
}

struct Solution;
impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        return nums
            .iter()
            .map(|&x| {
                x.to_string()
                    .chars()
                    .rev()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap()
            })
            .zip(nums.iter())
            .map(|(x, y)| x + y)
            .fold(std::collections::HashMap::new(), |mut acc, x| {
                *acc.entry(x).or_insert(0) += 1;
                acc
            })
            .values()
            .map(|&x| x * (x - 1) / 2)
            .sum::<i32>() as i32;
    }
}
