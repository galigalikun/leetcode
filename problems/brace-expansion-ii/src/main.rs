use std::collections::HashSet;

fn main() {
    // Test case 1
    assert_eq!(
        Solution::brace_expansion_ii("{a,b}{c,{d,e}}".to_string()),
        vec!["ac", "ad", "ae", "bc", "bd", "be"]
    );
    
    // Test case 2
    assert_eq!(
        Solution::brace_expansion_ii("{{a,z},a{b,c},{ab,z}}".to_string()),
        vec!["a", "ab", "ac", "z"]
    );
    
    // Test case 3: Simple single letter
    assert_eq!(
        Solution::brace_expansion_ii("a".to_string()),
        vec!["a"]
    );
    
    // Test case 4: Simple braces
    assert_eq!(
        Solution::brace_expansion_ii("{a}".to_string()),
        vec!["a"]
    );
    
    // Test case 5: Simple union
    assert_eq!(
        Solution::brace_expansion_ii("{a,b,c}".to_string()),
        vec!["a", "b", "c"]
    );
    
    println!("All tests passed!");
}

struct Solution;

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let chars: Vec<char> = expression.chars().collect();
        let (result, _) = Self::parse_sequence(&chars, 0, false);
        let mut sorted: Vec<String> = result.into_iter().collect();
        sorted.sort();
        sorted
    }

    // Parse a sequence of concatenated expressions
    // is_union_context: true if we're inside a union (should stop at commas and braces)
    fn parse_sequence(chars: &[char], start: usize, is_union_context: bool) -> (HashSet<String>, usize) {
        let mut pos = start;
        let mut result: HashSet<String> = HashSet::from([String::new()]);

        while pos < chars.len() {
            if is_union_context && (chars[pos] == ',' || chars[pos] == '}') {
                break;
            }
            if chars[pos] == '}' {
                break;
            }

            let (next_set, next_pos) = Self::parse_term(chars, pos);
            let mut new_result = HashSet::new();
            for r in &result {
                for n in &next_set {
                    new_result.insert(format!("{}{}", r, n));
                }
            }
            result = new_result;
            pos = next_pos;
        }

        (result, pos)
    }

    // Parse a single term (either a letter sequence or a union in braces)
    fn parse_term(chars: &[char], start: usize) -> (HashSet<String>, usize) {
        if start >= chars.len() {
            return (HashSet::new(), start);
        }

        if chars[start] == '{' {
            Self::parse_union(chars, start + 1)
        } else {
            Self::parse_letters(chars, start)
        }
    }

    // Parse a union (comma-separated sequences within braces)
    fn parse_union(chars: &[char], start: usize) -> (HashSet<String>, usize) {
        let mut pos = start;
        let mut result = HashSet::new();

        loop {
            // Inside a union, parse sequences that stop at commas or closing braces
            let (seq_result, next_pos) = Self::parse_sequence(chars, pos, true);
            for item in seq_result {
                result.insert(item);
            }
            pos = next_pos;

            if pos >= chars.len() {
                break;
            }

            if chars[pos] == ',' {
                pos += 1;
            } else if chars[pos] == '}' {
                pos += 1;
                break;
            } else {
                break;
            }
        }

        (result, pos)
    }

    // Parse a sequence of lowercase letters
    fn parse_letters(chars: &[char], start: usize) -> (HashSet<String>, usize) {
        let mut pos = start;
        let mut word = String::new();

        while pos < chars.len() && chars[pos].is_ascii_lowercase() {
            word.push(chars[pos]);
            pos += 1;
        }

        (HashSet::from([word]), pos)
    }
}

