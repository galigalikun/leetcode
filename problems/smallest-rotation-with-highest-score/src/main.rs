fn main() {
    assert_eq!(Solution::best_rotation(vec![2, 3, 1, 4, 0]), 3);
    assert_eq!(Solution::best_rotation(vec![1, 3, 0, 2, 4]), 0);
}

struct Solution {}

impl Solution {
    pub fn best_rotation(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let mut diff = vec![0_i32; n + 1];

        for (i, &num) in nums.iter().enumerate() {
            let value = num as usize;
            if value == 0 {
                continue;
            }

            let left = (i + n - value + 1) % n;
            let right = (i + 1) % n;

            if left < right {
                diff[left] += 1;
                diff[right] -= 1;
            } else {
                diff[0] += 1;
                diff[right] -= 1;
                diff[left] += 1;
            }
        }

        let mut bad = 0_i32;
        let mut best_k = 0;
        let mut min_bad = i32::MAX;

        for (k, delta) in diff.iter().take(n).enumerate() {
            bad += *delta;
            if bad < min_bad {
                min_bad = bad;
                best_k = k;
            }
        }

        best_k as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn sample_cases() {
        assert_eq!(Solution::best_rotation(vec![2, 3, 1, 4, 0]), 3);
        assert_eq!(Solution::best_rotation(vec![1, 3, 0, 2, 4]), 0);
    }

    #[test]
    fn prompt_case() {
        assert_eq!(Solution::best_rotation(vec![2, 4, 1, 3, 0]), 0);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::best_rotation(vec![0]), 0);
    }
}
