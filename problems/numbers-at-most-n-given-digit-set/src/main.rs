fn main() {
    assert_eq!(
        Solution::at_most_n_given_digit_set(
            vec![
                "1".to_string(),
                "3".to_string(),
                "5".to_string(),
                "7".to_string()
            ],
            100
        ),
        20
    );
    assert_eq!(
        Solution::at_most_n_given_digit_set(
            vec!["1".to_string(), "4".to_string(), "9".to_string()],
            1000000000
        ),
        29523
    );
    assert_eq!(
        Solution::at_most_n_given_digit_set(vec!["7".to_string()], 8),
        1
    );
}

struct Solution;
impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let n_chars: Vec<char> = n.to_string().chars().collect();
        let target_len = n_chars.len();
        let available_digits: Vec<char> = digits
            .iter()
            .map(|digit| digit.chars().next().unwrap_or('0'))
            .collect();

        let base = available_digits.len() as i32;
        let mut total = 0_i32;

        for len in 1..target_len {
            total += base.pow(len as u32);
        }

        for (index, current_digit) in n_chars.iter().enumerate() {
            let mut smaller_count = 0_i32;
            let mut has_equal = false;

            for &digit in &available_digits {
                if digit < *current_digit {
                    smaller_count += 1;
                } else if digit == *current_digit {
                    has_equal = true;
                }
            }

            let remaining = (target_len - index - 1) as u32;
            total += smaller_count * base.pow(remaining);

            if !has_equal {
                return total;
            }
        }

        total + 1
    }
}
