fn main() {
    assert_eq!(Solution::sum_subarray_mins(vec![3,1,2,4]), 17);
    assert_eq!(Solution::sum_subarray_mins(vec![11,81,94,43,3]), 444);
}

struct Solution;
impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = arr.len();

        let mut prev_less = vec![-1_i64; n];
        let mut next_less_or_equal = vec![n as i64; n];
        let mut stack: Vec<usize> = Vec::new();

        for i in 0..n {
            while let Some(&j) = stack.last() {
                if arr[j] > arr[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
            if let Some(&j) = stack.last() {
                prev_less[i] = j as i64;
            }
            stack.push(i);
        }

        stack.clear();
        for i in (0..n).rev() {
            while let Some(&j) = stack.last() {
                if arr[j] >= arr[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
            if let Some(&j) = stack.last() {
                next_less_or_equal[i] = j as i64;
            }
            stack.push(i);
        }

        let mut answer: i64 = 0;
        for i in 0..n {
            let left = i as i64 - prev_less[i];
            let right = next_less_or_equal[i] - i as i64;
            let contribution = (arr[i] as i64) * left % MOD * right % MOD;
            answer = (answer + contribution) % MOD;
        }

        answer as i32
    }
}
