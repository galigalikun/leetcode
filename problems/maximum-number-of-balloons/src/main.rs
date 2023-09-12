fn main() {
    assert_eq!(Solution::max_number_of_balloons("nlaebolko".to_string()), 1);
    assert_eq!(
        Solution::max_number_of_balloons("loonbalxballpoon".to_string()),
        2
    );
    assert_eq!(Solution::max_number_of_balloons("leetcode".to_string()), 0);
}

struct Solution;
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut count = [0; 26];
        for c in text.chars() {
            count[c as usize - 'a' as usize] += 1;
        }
        let mut min = count[1]; // b
        min = std::cmp::min(min, count[0]); // a
        min = std::cmp::min(min, count[11] / 2); // l
        min = std::cmp::min(min, count[14] / 2); // o
        min = std::cmp::min(min, count[13]); // n
        return min;
    }
}
