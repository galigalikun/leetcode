fn main() {
    assert_eq!(Solution::min_swaps("111000".to_string()), 1);
    assert_eq!(Solution::min_swaps("010".to_string()), 0);
    assert_eq!(Solution::min_swaps("1110".to_string()), -1);
}

struct Solution;
impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut ones: i32 = 0;
        let mut zeros: i32 = 0;

        for c in s.chars() {
            if c == '1' {
                ones += 1;
            } else {
                zeros += 1;
            }
        }

        if (ones - zeros).abs() > 1 {
            return -1; // Impossible to balance
        }

        let target = if ones >= zeros { '1' } else { '0' };
        let mut swaps = 0;
        let mut expected = target;

        for c in s.chars() {
            if c != expected {
                swaps += 1;
            }
            expected = if expected == '1' { '0' } else { '1' };
        }

        (swaps + 1) / 2 // Each swap fixes two positions
    }
}
