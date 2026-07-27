use std::collections::HashMap;

fn main() {
    let first = Solution::soup_servings(50);
    let second = Solution::soup_servings(100);
    assert!((first - 0.625).abs() < 1e-9);
    assert!((second - 0.71875).abs() < 1e-9);
}

struct Solution {}

impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        if n > 4_800 {
            return 1.0;
        }

        let units = (n + 24) / 25;
        let mut memo = HashMap::new();
        Self::dfs(units, units, &mut memo)
    }

    fn dfs(a: i32, b: i32, memo: &mut HashMap<(i32, i32), f64>) -> f64 {
        if a <= 0 && b <= 0 {
            return 0.5;
        }
        if a <= 0 {
            return 1.0;
        }
        if b <= 0 {
            return 0.0;
        }

        if let Some(&cached) = memo.get(&(a, b)) {
            return cached;
        }

        let result = 0.25
            * (Self::dfs(a - 4, b, memo)
                + Self::dfs(a - 3, b - 1, memo)
                + Self::dfs(a - 2, b - 2, memo)
                + Self::dfs(a - 1, b - 3, memo));

        memo.insert((a, b), result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_cases() {
        assert!((Solution::soup_servings(50) - 0.625).abs() < 1e-9);
        assert!((Solution::soup_servings(100) - 0.71875).abs() < 1e-9);
    }

    #[test]
    fn large_n_is_almost_one() {
        assert!((Solution::soup_servings(4_801) - 1.0).abs() < 1e-12);
    }
}
