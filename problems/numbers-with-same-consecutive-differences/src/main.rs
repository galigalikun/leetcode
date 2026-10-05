fn main() {
    assert_eq!(Solution::nums_same_consec_diff(3, 7), vec![181,292,707,818,929]);
    assert_eq!(Solution::nums_same_consec_diff(2, 1), vec![10,12,21,23,32,34,43,45,54,56,65,67,76,78,87,89,98]);
    assert_eq!(Solution::nums_same_consec_diff(1, 0), vec![0,1,2,3,4,5,6,7,8,9]);
    assert_eq!(Solution::nums_same_consec_diff(2, 0), vec![11,22,33,44,55,66,77,88,99]);
}

struct Solution;
impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        if n <= 0 || k < 0 {
            return vec![];
        }

        let mut result = Vec::new();

        if n == 1 {
            for digit in 0..=9 {
                result.push(digit);
            }
            return result;
        }

        fn build_numbers(path: &mut Vec<i32>, remaining: i32, last: i32, k: i32, result: &mut Vec<i32>) {
            if remaining == 0 {
                let mut value = 0;
                for &digit in path.iter() {
                    value = value * 10 + digit;
                }
                result.push(value);
                return;
            }

            let candidates = if k == 0 {
                vec![last]
            } else {
                vec![last + k, last - k]
            };

            for next in candidates {
                if next < 0 || next > 9 {
                    continue;
                }
                path.push(next);
                build_numbers(path, remaining - 1, next, k, result);
                path.pop();
            }
        }

        for first in 1..=9 {
            let mut path = vec![first];
            build_numbers(&mut path, n - 1, first, k, &mut result);
        }

        result.sort_unstable();
        result
    }
}
