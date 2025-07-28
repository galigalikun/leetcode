fn main() {
    assert_eq!(Solution::sum_game("5023".to_string()), false);
    assert_eq!(Solution::sum_game("25??".to_string()), true);
    assert_eq!(Solution::sum_game("?3295???".to_string()), false);
}

struct Solution;
impl Solution {
    pub fn sum_game(num: String) -> bool {
        let n = num.len();
        let mut left_sum = 0;
        let mut right_sum = 0;
        let mut left_question_marks = 0;
        let mut right_question_marks = 0;
        for i in 0..n / 2 {
            let left_char = num.chars().nth(i).unwrap();
            let right_char = num.chars().nth(n - 1 - i).unwrap();
            if left_char == '?' {
                left_question_marks += 1;
            } else {
                left_sum += left_char.to_digit(10).unwrap();
            }
            if right_char == '?' {
                right_question_marks += 1;
            } else {
                right_sum += right_char.to_digit(10).unwrap();
            }
        }
        let total_question_marks = left_question_marks + right_question_marks;
        let total_difference = (left_sum as i32 - right_sum as i32).abs();
        if total_question_marks == 0 {
            return left_sum == right_sum;
        }
        if total_question_marks == 1 {
            return total_difference == 0;
        }
        if total_question_marks == 2 {
            return total_difference <= 9;
        }
        if total_question_marks % 2 == 0 {
            return total_difference % 9 == 0;
        } else {
            return false;
        }
    }
}
