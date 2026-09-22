fn main() {
    let _ = Solution::reorder_log_files(Vec::new());
}

struct Solution;
impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut letter_logs: Vec<String> = Vec::new();
        let mut digit_logs: Vec<String> = Vec::new();

        for log in logs {
            let (_, content) = log.split_once(' ').unwrap();
            if content.chars().next().unwrap().is_ascii_digit() {
                digit_logs.push(log);
            } else {
                letter_logs.push(log);
            }
        }

        letter_logs.sort_by(|a, b| {
            let mut a_split = a.splitn(2, ' ');
            let mut b_split = b.splitn(2, ' ');
            let a_identifier = a_split.next().unwrap();
            let a_content = a_split.next().unwrap();
            let b_identifier = b_split.next().unwrap();
            let b_content = b_split.next().unwrap();
            let cmp = a_content.cmp(b_content);
            if cmp == std::cmp::Ordering::Equal {
                a_identifier.cmp(b_identifier)
            } else {
                cmp
            }
        });

        letter_logs.extend(digit_logs);
        letter_logs
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn s(input: &[&str]) -> Vec<String> {
        input.iter().map(|v| v.to_string()).collect()
    }

    #[test]
    fn sample_case_1() {
        let logs = s(&[
            "dig1 8 1 5 1",
            "let1 art can",
            "dig2 3 6",
            "let2 own kit dig",
            "let3 art zero",
        ]);
        let expected = s(&[
            "let1 art can",
            "let3 art zero",
            "let2 own kit dig",
            "dig1 8 1 5 1",
            "dig2 3 6",
        ]);

        assert_eq!(Solution::reorder_log_files(logs), expected);
    }

    #[test]
    fn sample_case_2() {
        let logs = s(&[
            "a1 9 2 3 1",
            "g1 act car",
            "zo4 4 7",
            "ab1 off key dog",
            "a8 act zoo",
        ]);
        let expected = s(&[
            "g1 act car",
            "a8 act zoo",
            "ab1 off key dog",
            "a1 9 2 3 1",
            "zo4 4 7",
        ]);

        assert_eq!(Solution::reorder_log_files(logs), expected);
    }
}
