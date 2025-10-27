fn main() {
    assert_eq!(
        Solution::smallest_subsequence("leet".to_string(), 3, 'e', 1),
        "eet"
    );
    assert_eq!(
        Solution::smallest_subsequence("leetcode".to_string(), 4, 'e', 2),
        "ecde"
    );
    assert_eq!(
        Solution::smallest_subsequence("bb".to_string(), 2, 'b', 2),
        "bb"
    );
}

struct Solution;
impl Solution {
    pub fn smallest_subsequence(s: String, k: i32, letter: char, repetition: i32) -> String {
        let mut stack = Vec::new();
        let (mut letter_count, mut total_letter) = (0, 0);
        for ch in s.chars() {
            if ch == letter {
                total_letter += 1;
            }
        }
        for ch in s.chars() {
            while let Some(&last) = stack.last() {
                if ch < last
                    && (stack.len() + s.len() - stack.len() - 1 >= k as usize)
                    && (if last == letter {
                        letter_count + total_letter - 1 >= repetition
                    } else {
                        letter_count + total_letter >= repetition
                    })
                {
                    if last == letter {
                        letter_count -= 1;
                    }
                    stack.pop();
                } else {
                    break;
                }
            }
            if stack.len() < k as usize {
                if ch == letter {
                    stack.push(ch);
                    letter_count += 1;
                } else if (k as usize - stack.len()) > (repetition - letter_count) as usize {
                    stack.push(ch);
                }
            }
            if ch == letter {
                total_letter -= 1;
            }
        }
        if letter_count < repetition {
            let mut to_add = repetition - letter_count;
            for i in (0..stack.len()).rev() {
                if to_add == 0 {
                    break;
                }
                if stack[i] != letter {
                    stack[i] = letter;
                    to_add -= 1;
                }
            }
        }
        return stack.into_iter().collect();
    }
}
