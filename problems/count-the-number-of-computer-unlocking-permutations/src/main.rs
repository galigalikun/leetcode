fn main() {
    assert_eq!(Solution::count_permutations(vec![1, 2, 3]), 2);
    assert_eq!(Solution::count_permutations(vec![3, 3, 3, 4, 4, 4]), 0);
}

struct Solution;
impl Solution {
    pub fn count_permutations(complexity: Vec<i32>) -> i32 {
        let n = complexity.len();
        let mut sorted = complexity.clone();
        sorted.sort_unstable();
        for i in 0..n {
            if sorted[i] > i as i32 {
                return 0;
            }
        }
        let modulo = 1_000_000_007;
        let mut result = 1i64;
        for i in 0..n {
            let choices = i as i32 - sorted[i] + 1;
            result = (result * choices as i64) % modulo;
        }
        result as i32
    }
}
