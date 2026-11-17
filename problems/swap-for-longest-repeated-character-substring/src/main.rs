use std::collections::HashMap;

fn main() {
    assert_eq!(Solution::max_rep_opt1("ababa".to_string()), 3);
    assert_eq!(Solution::max_rep_opt1("aaabaaa".to_string()), 6);
    assert_eq!(Solution::max_rep_opt1("aaaaa".to_string()), 5);
    assert_eq!(Solution::max_rep_opt1("aabbaa".to_string()), 3);
    assert_eq!(Solution::max_rep_opt1("abca".to_string()), 2);
    assert_eq!(Solution::max_rep_opt1("abcada".to_string()), 3);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn known_examples() {
        assert_eq!(Solution::max_rep_opt1("ababa".to_string()), 3);
        assert_eq!(Solution::max_rep_opt1("aaabaaa".to_string()), 6);
        assert_eq!(Solution::max_rep_opt1("aaaaa".to_string()), 5);
        assert_eq!(Solution::max_rep_opt1("aabbaa".to_string()), 3);
        assert_eq!(Solution::max_rep_opt1("abca".to_string()), 2);
        assert_eq!(Solution::max_rep_opt1("abcada".to_string()), 3);
    }
}

struct Solution;
impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        let text: Vec<char> = text.chars().collect();
        let mut counts = HashMap::new();
        for &ch in &text {
            *counts.entry(ch).or_insert(0usize) += 1;
        }

        let mut left = 0usize;
        let mut right = 0usize;
        let mut result = 0i32;

        while right < text.len() {
            if right == text.len() - 1 || text[right + 1] != text[right] {
                let run_len = right - left + 1;
                let total = *counts.get(&text[left]).unwrap();

                if total > run_len {
                    if left == 0 {
                        result = result.max(run_len as i32);
                    } else if left == 1 {
                        result = result.max((run_len + 1) as i32);
                    } else {
                        let mut idx = left - 2;
                        let mut before = 0usize;
                        loop {
                            if text[idx] == text[left] {
                                before += 1;
                                if idx == 0 {
                                    break;
                                }
                                idx -= 1;
                            } else {
                                break;
                            }
                        }
                        result = result.max(std::cmp::min(total, run_len + 1 + before) as i32);
                    }

                    if right + 1 < text.len() {
                        let mut idx = right + 2;
                        let mut after = 0usize;
                        while idx < text.len() && text[idx] == text[right] {
                            after += 1;
                            idx += 1;
                        }
                        result = result.max(std::cmp::min(total, run_len + 1 + after) as i32);
                    }
                } else {
                    result = result.max(run_len as i32);
                }

                left = right + 1;
                right += 1;
            } else {
                right += 1;
            }
        }

        result
    }
}
