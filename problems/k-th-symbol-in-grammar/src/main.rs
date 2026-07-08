fn main() {
    assert_eq!(Solution::kth_grammar(1, 1), 0);
    assert_eq!(Solution::kth_grammar(2, 1), 0);
    assert_eq!(Solution::kth_grammar(2, 2), 1);
    assert_eq!(Solution::kth_grammar(3, 1), 0);
    assert_eq!(Solution::kth_grammar(3, 2), 1);
    assert_eq!(Solution::kth_grammar(3, 3), 1);
    assert_eq!(Solution::kth_grammar(3, 4), 0);
}

struct Solution {}

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 0;
        }

        let parent = Self::kth_grammar(n - 1, (k + 1) / 2);
        if k % 2 == 1 {
            parent
        } else {
            1 - parent
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn handles_base_case() {
        assert_eq!(Solution::kth_grammar(1, 1), 0);
    }

    #[test]
    fn handles_second_row() {
        assert_eq!(Solution::kth_grammar(2, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 2), 1);
    }

    #[test]
    fn handles_third_row() {
        assert_eq!(Solution::kth_grammar(3, 1), 0);
        assert_eq!(Solution::kth_grammar(3, 2), 1);
        assert_eq!(Solution::kth_grammar(3, 3), 1);
        assert_eq!(Solution::kth_grammar(3, 4), 0);
    }

    #[test]
    fn handles_larger_case() {
        assert_eq!(Solution::kth_grammar(4, 5), 1);
    }
}
