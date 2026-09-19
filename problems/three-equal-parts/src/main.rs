struct Solution;
impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let n = arr.len();
        let ones = arr.iter().filter(|&&x| x == 1).count();

        if ones == 0 {
            return vec![0, 2];
        }

        if ones % 3 != 0 {
            return vec![-1, -1];
        }

        let ones_per_part = ones / 3;
        let mut first = 0usize;
        let mut second = 0usize;
        let mut third = 0usize;
        let mut seen = 0usize;

        for (idx, &bit) in arr.iter().enumerate() {
            if bit == 1 {
                seen += 1;
                if seen == 1 {
                    first = idx;
                } else if seen == ones_per_part + 1 {
                    second = idx;
                } else if seen == 2 * ones_per_part + 1 {
                    third = idx;
                    break;
                }
            }
        }

        while third < n && arr[first] == arr[second] && arr[second] == arr[third] {
            first += 1;
            second += 1;
            third += 1;
        }

        if third == n {
            vec![(first - 1) as i32, second as i32]
        } else {
            vec![-1, -1]
        }
    }
}

fn main() {
    assert_eq!(Solution::three_equal_parts(vec![1, 0, 1, 0, 1]), vec![0, 3]);
    assert_eq!(
        Solution::three_equal_parts(vec![1, 1, 0, 1, 1]),
        vec![-1, -1]
    );
    assert_eq!(Solution::three_equal_parts(vec![1, 1, 0, 0, 1]), vec![0, 2]);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn returns_valid_split_for_sample_case() {
        let result = Solution::three_equal_parts(vec![1, 0, 1, 0, 1]);
        assert_eq!(result, vec![0, 3]);
    }

    #[test]
    fn returns_no_solution_when_ones_not_divisible_by_three() {
        let result = Solution::three_equal_parts(vec![1, 1, 0, 1, 1]);
        assert_eq!(result, vec![-1, -1]);
    }

    #[test]
    fn handles_all_zeroes() {
        let result = Solution::three_equal_parts(vec![0, 0, 0, 0, 0]);
        assert_eq!(result, vec![0, 2]);
    }

    #[test]
    fn handles_leading_zeroes() {
        let result = Solution::three_equal_parts(vec![0, 1, 0, 1, 0, 1]);
        assert_eq!(result, vec![1, 4]);
    }
}
