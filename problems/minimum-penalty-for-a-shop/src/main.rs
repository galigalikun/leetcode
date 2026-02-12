fn main() {
    assert_eq!(Solution::best_closing_time("YYNY".to_string()), 2);
    assert_eq!(Solution::best_closing_time("NNNNN".to_string()), 0);
    assert_eq!(Solution::best_closing_time("YYYY".to_string()), 4);
}

struct Solution;
impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let chars: Vec<char> = customers.chars().collect();
        let n = chars.len();
        let mut prefix_y = vec![0; n + 1];
        let mut suffix_n = vec![0; n + 1];

        for i in 0..n {
            prefix_y[i + 1] = prefix_y[i] + if chars[i] == 'Y' { 1 } else { 0 };
        }

        for i in (0..n).rev() {
            suffix_n[i] = suffix_n[i + 1] + if chars[i] == 'N' { 1 } else { 0 };
        }

        let mut min_penalty = std::i32::MAX;
        let mut best_hour = 0;

        for hour in 0..=n {
            let penalty = prefix_y[hour] + suffix_n[hour];
            if penalty < min_penalty {
                min_penalty = penalty;
                best_hour = hour as i32;
            }
        }

        best_hour
    }
}
