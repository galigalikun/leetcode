fn main() {
    assert_eq!(Solution::min_flips("10111".to_string()), 3);
    assert_eq!(Solution::min_flips("101".to_string()), 3);
    assert_eq!(Solution::min_flips("00000".to_string()), 0);
}

struct Solution;
impl Solution {
    pub fn min_flips(target: String) -> i32 {
        let target = target.chars().collect::<Vec<char>>();
        let mut count = 0;
        let mut i = 0;
        while i < target.len() {
            if target[i] == '1' {
                count += 1;
                while i < target.len() && target[i] == '1' {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
        count
    }
}
