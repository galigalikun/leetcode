fn main() {
    assert_eq!(
        Solution::can_transform("RXXLRXRXL".to_string(), "XRLXXRRLX".to_string()),
        true
    );
    assert_eq!(Solution::can_transform("X".to_string(), "L".to_string()), false);
}

struct Solution {}

impl Solution {
    pub fn can_transform(start: String, end: String) -> bool {
        if start.len() != end.len() {
            return false;
        }

        let s = start.as_bytes();
        let e = end.as_bytes();
        let n = s.len();

        let mut i = 0;
        let mut j = 0;

        while i < n || j < n {
            while i < n && s[i] == b'X' {
                i += 1;
            }
            while j < n && e[j] == b'X' {
                j += 1;
            }

            if i == n || j == n {
                return i == n && j == n;
            }

            if s[i] != e[j] {
                return false;
            }

            // L can only move left, R can only move right.
            if s[i] == b'L' && i < j {
                return false;
            }
            if s[i] == b'R' && i > j {
                return false;
            }

            i += 1;
            j += 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn sample_true() {
        assert!(Solution::can_transform(
            "RXXLRXRXL".to_string(),
            "XRLXXRRLX".to_string()
        ));
    }

    #[test]
    fn sample_false() {
        assert!(!Solution::can_transform("X".to_string(), "L".to_string()));
    }

    #[test]
    fn order_of_non_x_must_match() {
        assert!(!Solution::can_transform(
            "RLX".to_string(),
            "XLR".to_string()
        ));
    }

    #[test]
    fn l_cannot_move_right() {
        assert!(!Solution::can_transform(
            "LX".to_string(),
            "XL".to_string()
        ));
    }

    #[test]
    fn r_cannot_move_left() {
        assert!(!Solution::can_transform(
            "XR".to_string(),
            "RX".to_string()
        ));
    }
}
