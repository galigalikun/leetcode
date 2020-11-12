fn main() {
    assert_eq!(
        Solution::full_justify(
            vec![
                "This".to_string(),
                "is".to_string(),
                "an".to_string(),
                "example".to_string(),
                "of".to_string(),
                "text".to_string(),
                "justification.".to_string()
            ],
            16
        ),
        vec![
            "This    is    an".to_string(),
            "example  of text".to_string(),
            "justification.  ".to_string()
        ]
    );
    assert_eq!(
        Solution::full_justify(
            vec![
                "What".to_string(),
                "must".to_string(),
                "be".to_string(),
                "acknowledgment".to_string(),
                "shall".to_string(),
                "be".to_string()
            ],
            16
        ),
        vec![
            "What   must   be".to_string(),
            "acknowledgment  ".to_string(),
            "shall be        ".to_string()
        ]
    );
    assert_eq!(
        Solution::full_justify(
            vec![
                "Science".to_string(),
                "is".to_string(),
                "what".to_string(),
                "we".to_string(),
                "understand".to_string(),
                "well".to_string(),
                "enough".to_string(),
                "to".to_string(),
                "explain".to_string(),
                "to".to_string(),
                "a".to_string(),
                "computer.".to_string(),
                "Art".to_string(),
                "is".to_string(),
                "everything".to_string(),
                "else".to_string(),
                "we".to_string(),
                "do".to_string()
            ],
            20
        ),
        vec![
            "Science  is  what we".to_string(),
            "understand      well".to_string(),
            "enough to explain to".to_string(),
            "a  computer.  Art is".to_string(),
            "everything  else  we".to_string(),
            "do                  ".to_string()
        ]
    );
}

pub struct Solution {}
impl Solution {
    fn full_justify_format(words: Vec<String>, word_count: usize, max_width: i32) -> String {
        let mut word = "".to_string();
        let (width, mut over) = if words.len() > 1 {
            (
                (max_width as usize - word_count) / (words.len() - 1),
                (max_width as usize - word_count) % (words.len() - 1),
            )
        } else {
            (max_width as usize - word_count, 0)
        };
        for i in 0..words.len() {
            if i == words.len() - 1 && i > 0 {
                word = format!("{}{}", word, words[i]);
            } else if over > 0 {
                word = format!(
                    "{}{:width$}",
                    word,
                    words[i],
                    width = words[i].as_str().chars().count() + width + 1
                );
                over -= 1;
            } else {
                word = format!(
                    "{}{:width$}",
                    word,
                    words[i],
                    width = words[i].as_str().chars().count() + width
                );
            }
        }
        return word;
    }
    fn full_justify_last(words: Vec<String>, max_width: i32) -> String {
        let mut word = "".to_string();
        for i in 0..words.len() {
            if i == words.len() - 1 {
                let count = max_width as usize - word.as_str().chars().count();
                word = format!("{}{:width$}", word, words[i], width = count);
            } else {
                word = format!("{}{} ", word, words[i]);
            }
        }
        return word;
    }
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut word_count = 0;
        let mut work: Vec<String> = Vec::new();
        for w in words {
            word_count += w.as_str().chars().count();

            if (word_count + work.len()-1) as i32 >= max_width {
                result.push(Solution::full_justify_format(
                    work.clone(),
                    word_count - w.as_str().chars().count(),
                    max_width,
                ));
                word_count = w.as_str().chars().count();
                work.clear();
                work.push(w);
            } else {
                work.push(w);
            }
        }
        if word_count > 0 {
            result.push(Solution::full_justify_last(work, max_width));
        }
        return result;
    }
}
