fn main() {
    assert_eq!(Solution::num_dup_digits_at_most_n(20), 1);
    assert_eq!(Solution::num_dup_digits_at_most_n(100), 10);
    assert_eq!(Solution::num_dup_digits_at_most_n(1000), 262);
}

struct Solution;
impl Solution {
    fn permutation(n: i32, m: i32) -> i32 {
        let mut result = 1;
        for i in 0..m {
            result *= n - i;
        }
        return result;
    }
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        let mut n = n;
        let mut digits = Vec::new();
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.reverse();
        let mut result = 0;
        for i in 1..digits.len() {
            result += 9 * Self::permutation(9, i as i32 - 1);
        }
        let mut used = [false; 10];
        for i in 0..digits.len() {
            let start = if i == 0 { 1 } else { 0 };
            for j in start..digits[i] {
                if !used[j as usize] {
                    result += Self::permutation(9 - i as i32, digits.len() as i32 - i as i32 - 1);
                }
            }
            if used[digits[i] as usize] {
                break;
            }
            used[digits[i] as usize] = true;
            if i == digits.len() - 1 {
                result += 1;
            }
        }
        return (n - result) as i32;
    }
}
