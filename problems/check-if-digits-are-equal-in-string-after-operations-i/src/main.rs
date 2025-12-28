fn main() {
    assert_eq!(Solution::has_same_digits("3902".to_string()), true);
    assert_eq!(Solution::has_same_digits("34789".to_string()), false);
    assert_eq!(Solution::has_same_digits("323".to_string()), true);
}

struct Solution;
impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut s = s;
        loop {
            let mut result = String::new();
            for i in 0..s.len() - 1 {
                let a = s[i..i + 1].parse::<u32>().unwrap();
                let b = s[i + 1..i + 2].parse::<u32>().unwrap();
                let c = (a + b) % 10;
                result.push_str(&c.to_string());
            }
            if result.len() == 2 {
                return result.as_bytes()[0] == result.as_bytes()[1];
            }
            if result.len() < 2 {
                return false;
            }
            s = result;
        }
    }
}
