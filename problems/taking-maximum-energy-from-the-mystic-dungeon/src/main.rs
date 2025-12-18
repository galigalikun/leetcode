fn main() {
    assert_eq!(Solution::maximum_energy(vec![5, 2, -10, -5, 1], 3), 3);
    assert_eq!(Solution::maximum_energy(vec![-2, -3, -1], 2), -1);
}

struct Solution;
impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let n = energy.len() as i32;
        if n <= k {
            return energy.iter().max().cloned().unwrap_or(0);
        }
        return energy.iter().sum::<i32>() + k;
    }
}
