fn main() {
    assert_eq!(
        Solution::make_fancy_string("leeetcode".to_string()),
        "leetcode".to_string()
    );
    assert_eq!(
        Solution::make_fancy_string("aaabaaaa".to_string()),
        "aabaa".to_string()
    );
    assert_eq!(
        Solution::make_fancy_string("aab".to_string()),
        "aab".to_string()
    );
}

struct Solution;
impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        return s
            .chars()
            .fold((String::new(), 0, '\0'), |(mut acc, count, last), c| {
                if c == last {
                    if count < 2 {
                        acc.push(c);
                        (acc, count + 1, c)
                    } else {
                        (acc, count, last)
                    }
                } else {
                    acc.push(c);
                    (acc, 1, c)
                }
            })
            .0;
    }
}
