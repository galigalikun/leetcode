fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
}

pub struct Solution {}
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        for i in 0..numbers.len() - 1 {
            for j in 0..=i {
                let sum = numbers[j] + numbers[i + 1];
                if sum == target {
                    result.push((j + 1) as i32);
                    result.push((i + 2) as i32);
                }
            }
        }

        return result;
    }
}
