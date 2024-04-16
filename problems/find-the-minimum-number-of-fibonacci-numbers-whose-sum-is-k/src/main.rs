fn main() {
    assert_eq!(Solution::find_min_fibonacci_numbers(7), 2);
    assert_eq!(Solution::find_min_fibonacci_numbers(10), 2);
    assert_eq!(Solution::find_min_fibonacci_numbers(19), 3);
}

struct Solution;
impl Solution {
    pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
        let mut fib = vec![1, 1];
        let mut i = 1;
        while fib[i] < k {
            fib.push(fib[i] + fib[i - 1]);
            i += 1;
        }
        let mut count = 0;
        let mut sum = 0;
        for i in (0..fib.len()).rev() {
            if sum + fib[i] <= k {
                sum += fib[i];
                count += 1;
            }
            if sum == k {
                break;
            }
        }
        return count;
    }
}
