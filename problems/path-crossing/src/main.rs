fn main() {
    assert_eq!(Solution::is_path_crossing("NES".to_string()), false);
    assert_eq!(Solution::is_path_crossing("NESWW".to_string()), true);
}

struct Solution;
impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        for c in path.chars() {
            if c == 'N' {
                y += 1;
            }
            if c == 'S' {
                y -= 1;
            }
            if c == 'E' {
                x += 1;
            }
            if c == 'W' {
                x -= 1;
            }
        }
        if x <= 0 && y <= 0 {
            return true;
        }
        return false;
    }
}
