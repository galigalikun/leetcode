fn main() {
    assert_eq!(
        Solution::score_of_students("7+3*1*2".to_string(), vec![20, 13, 42]),
        7
    );
    assert_eq!(
        Solution::score_of_students("3+5*2".to_string(), vec![13, 0, 10, 13, 13, 16, 16]),
        19
    );
    assert_eq!(
        Solution::score_of_students("6+0*1".to_string(), vec![12, 9, 6, 4, 8, 6]),
        10
    );
}

struct Solution;
impl Solution {
    pub fn score_of_students(s: String, answers: Vec<i32>) -> i32 {
        let mut score = 0;
        for answer in answers {
            if answer == Solution::evaluate_expression(&s) {
                score += 5;
            } else if Solution::is_valid_answer(&s, answer) {
                score += 2;
            }
        }
        score
    }

    fn evaluate_expression(s: &str) -> i32 {
        let mut stack = Vec::new();
        let mut num = 0;
        let mut op = '+';

        for c in s.chars() {
            if c.is_digit(10) {
                num = num * 10 + c.to_digit(10).unwrap() as i32;
            } else if c == '+' || c == '*' {
                match op {
                    '+' => stack.push(num),
                    '*' => stack.push(stack.clone().pop().unwrap() * num),
                    _ => unreachable!(),
                }
                num = 0;
                op = c;
            }
        }
        match op {
            '+' => stack.push(num),
            '*' => stack.push(stack.clone().pop().unwrap() * num),
            _ => unreachable!(),
        }
        stack.iter().sum()
    }

    fn is_valid_answer(s: &str, answer: i32) -> bool {
        let mut stack = Vec::new();
        let mut num = 0;
        let mut op = '+';

        for c in s.chars() {
            if c.is_digit(10) {
                num = num * 10 + c.to_digit(10).unwrap() as i32;
            } else if c == '+' || c == '*' {
                match op {
                    '+' => stack.push(num),
                    '*' => stack.push(stack.clone().pop().unwrap() * num),
                    _ => unreachable!(),
                }
                num = 0;
                op = c;
            }
        }
        match op {
            '+' => stack.push(num),
            '*' => stack.push(stack.clone().pop().unwrap() * num),
            _ => unreachable!(),
        }
        stack.iter().sum::<i32>() == answer
    }
}
