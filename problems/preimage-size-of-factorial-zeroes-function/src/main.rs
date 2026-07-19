fn main() {
    assert_eq!(Solution::preimage_size_fzf(0), 5);
    assert_eq!(Solution::preimage_size_fzf(5), 0);
    assert_eq!(Solution::preimage_size_fzf(3), 5);
}

struct Solution {}

impl Solution {
    pub fn preimage_size_fzf(k: i32) -> i32 {
        let target = i64::from(k);
        let left = Self::lower_bound(target);
        let right = Self::lower_bound(target + 1);
        (right - left) as i32
    }

    fn lower_bound(target: i64) -> i64 {
        let mut low = 0_i64;
        let mut high = 5 * (target + 1);

        while low < high {
            let mid = low + (high - low) / 2;
            if Self::trailing_zeroes(mid) < target {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        low
    }

    fn trailing_zeroes(mut n: i64) -> i64 {
        let mut count = 0_i64;
        while n > 0 {
            n /= 5;
            count += n;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn sample_cases() {
        assert_eq!(Solution::preimage_size_fzf(0), 5);
        assert_eq!(Solution::preimage_size_fzf(5), 0);
        assert_eq!(Solution::preimage_size_fzf(3), 5);
    }

    #[test]
    fn boundary_around_gap() {
        assert_eq!(Solution::preimage_size_fzf(4), 5);
        assert_eq!(Solution::preimage_size_fzf(5), 0);
        assert_eq!(Solution::preimage_size_fzf(6), 5);
    }
}
