fn main() {
    assert_eq!(Solution::count_and_say(1), "1");
    assert_eq!(Solution::count_and_say(4), "1211");
}

struct Solution {}
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut s = String::from("1");
        for _ in 2..=n {
            if let Some(mut prev) = s.chars().nth(0) {
                let mut cnt = 0;
                let mut ss = String::from("");
                for c in s.chars() {
                    if prev != c {
                        ss = format!("{}{}{}", ss, cnt, prev);
                        cnt = 1;
                    } else {
                        cnt += 1;
                    }
                    prev = c;
                }
                if cnt > 0 {
                    s = format!("{}{}{}", ss, cnt, prev);
                } else {
                    s = ss;
                }
            }
        }
        return s.to_string();
    }
}
