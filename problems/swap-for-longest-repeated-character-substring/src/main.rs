fn main() {
    assert_eq!(Solution::max_rep_opt1("ababa".to_string()), 3);
    assert_eq!(Solution::max_rep_opt1("aaabaaa".to_string()), 6);
    assert_eq!(Solution::max_rep_opt1("aaaaa".to_string()), 5);
}

struct Solution;
impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        let mut max = 0;
        let mut count = 0;
        let mut prev = ' ';
        let mut prev_count = 0;
        let mut prev_prev = ' ';
        let mut prev_prev_count = 0;
        for c in text.chars() {
            if c == prev {
                count += 1;
            } else {
                if c == prev_prev {
                    prev_prev_count = prev_count;
                } else {
                    prev_prev_count = 0;
                }
                prev_prev = prev;
                prev_prev_count = prev_count;
                prev = c;
                count = 1;
            }
            prev_count = count;
            max = std::cmp::max(max, std::cmp::min(count + 1, prev_prev_count + 1));
        }
        return max;
    }
}
