fn main() {
    assert_eq!(Solution::title_to_number("A".to_string()), 1);
    assert_eq!(Solution::title_to_number("AB".to_string()), 28);
    assert_eq!(Solution::title_to_number("ZY".to_string()), 701);
}

struct Solution {}
impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let mut result = 0;
        let mut count = 0;
        for c in s.as_str().chars().rev() {
            let n = c as i32 - 64;
            result += (26 as i32).pow(count) * n;
            count += 1;
        }
        return result;
    }
}
