fn main() {
    assert_eq!(Solution::strange_printer(String::from("aaabbb")), 2);
    assert_eq!(Solution::strange_printer(String::from("aba")), 2);
    assert_eq!(
        Solution::strange_printer(String::from("aaaaaaaaaaaaaaaaaaaaa")),
        1
    );
}

// https://forketyfork.medium.com/strange-printer-6de1630821d2
struct Solution {}
impl Solution {
    fn helper(s: String, i: i32, j: i32) -> i32 {
        if i > j {
            return 0;
        }
        let first_letter = s.chars().nth(i as usize);
        let mut ans = 1 + Self::helper(s.clone(), i + 1, j);
        for k in i + 1..=j {
            if s.chars().nth(k as usize) == first_letter {
                let better_ans =
                    Self::helper(s.clone(), i, k - 1) + Self::helper(s.clone(), k + 1, j);
                ans = std::cmp::min(ans, better_ans);
            }
        }

        return ans;
    }
    pub fn strange_printer(s: String) -> i32 {
        return Self::helper(s.clone(), 0, s.len() as i32 - 1);
    }
}
