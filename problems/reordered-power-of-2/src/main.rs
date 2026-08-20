fn main() {
    assert_eq!(Solution::reordered_power_of2(1), true);
    assert_eq!(Solution::reordered_power_of2(10), false);
}

struct Solution;
impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let target = Self::digit_signature(n);
        (0..=30).any(|exp| Self::digit_signature(1_i32 << exp) == target)
    }

    fn digit_signature(value: i32) -> [u8; 10] {
        let digits = value.to_string();
        digits.bytes().fold([0; 10], |counts, byte| {
            let index = usize::from(byte - b'0');
            let mut next = counts;
            next[index] += 1;
            next
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn returns_true_when_reorder_can_form_power_of_two() {
        assert!(Solution::reordered_power_of2(46));
    }

    #[test]
    fn returns_false_when_reorder_cannot_form_power_of_two() {
        assert!(!Solution::reordered_power_of2(10));
    }

    #[test]
    fn handles_values_with_repeated_digits() {
        assert!(Solution::reordered_power_of2(821));
        assert!(!Solution::reordered_power_of2(122));
    }
}
