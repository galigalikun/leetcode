fn main() {
    assert_eq!(Solution::k_concatenation_max_sum(vec![1, 2], 3), 9);
    assert_eq!(Solution::k_concatenation_max_sum(vec![1, -2, 1], 5), 2);
    assert_eq!(Solution::k_concatenation_max_sum(vec![-1, -2], 7), 0);
}

struct Solution;
impl Solution {
    pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let mut total = 0_i64;
        let mut current = 0_i64;
        let mut max_subarray = 0_i64;

        for &value in &arr {
            let value = i64::from(value);
            total += value;
            current = (current + value).max(0);
            max_subarray = max_subarray.max(current);
        }

        if k == 1 {
            return (max_subarray % MOD) as i32;
        }

        let mut prefix = 0_i64;
        let mut max_prefix = 0_i64;
        for &value in &arr {
            prefix += i64::from(value);
            max_prefix = max_prefix.max(prefix);
        }

        let mut suffix = 0_i64;
        let mut max_suffix = 0_i64;
        for &value in arr.iter().rev() {
            suffix += i64::from(value);
            max_suffix = max_suffix.max(suffix);
        }

        let mut answer = max_subarray;
        if total > 0 {
            let middle = total * i64::from(k - 2);
            answer = answer.max(max_prefix + max_suffix + middle);
        } else {
            answer = answer.max(max_prefix + max_suffix);
        }

        (answer.max(0) % MOD) as i32
    }
}
