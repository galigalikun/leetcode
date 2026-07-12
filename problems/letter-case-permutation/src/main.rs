fn main() {
    assert_eq!(Solution::letter_case_permutation("a1b2".to_string()), vec!["a1b2","a1B2","A1b2","A1B2"]);
    assert_eq!(Solution::letter_case_permutation("3z4".to_string()), vec!["3z4","3Z4"]);
}

struct Solution{}
impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut ans = vec![String::new()];

        for c in s.chars() {
            if c.is_alphabetic() {
                ans = ans
                    .iter()
                    .flat_map(|prefix| {
                        let mut lower = prefix.clone();
                        lower.push(c.to_ascii_lowercase());

                        let mut upper = prefix.clone();
                        upper.push(c.to_ascii_uppercase());

                        [lower, upper]
                    })
                    .collect();
            } else {
                ans = ans
                    .iter()
                    .map(|prefix| {
                        let mut next = prefix.clone();
                        next.push(c);
                        next
                    })
                    .collect();
            }
        }

        ans
    }
}
