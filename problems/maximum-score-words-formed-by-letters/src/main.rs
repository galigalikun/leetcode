fn main() {
    assert_eq!(
        Solution::max_score_words(
            vec![
                "dog".to_string(),
                "cat".to_string(),
                "dad".to_string(),
                "good".to_string()
            ],
            vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
            vec![1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        ),
        23
    );
    assert_eq!(
        Solution::max_score_words(
            vec![
                "xxxz".to_string(),
                "ax".to_string(),
                "bx".to_string(),
                "cx".to_string()
            ],
            vec!['z', 'a', 'b', 'c', 'x', 'x', 'x'],
            vec![4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10]
        ),
        27
    );
    assert_eq!(
        Solution::max_score_words(
            vec!["leetcode".to_string()],
            vec!['l', 'e', 't', 'c', 'o', 'd'],
            vec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]
        ),
        0
    );
}

struct Solution;
impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut letters_count = vec![0; 26];
        for letter in letters {
            letters_count[(letter as u8 - 'a' as u8) as usize] += 1;
        }
        for i in 0..words.len() {
            let mut word_count = vec![0; 26];
            for letter in words[i].chars() {
                word_count[(letter as u8 - 'a' as u8) as usize] += 1;
            }
            let mut v_score = 0;
            for j in 0..26 {
                if word_count[j] > letters_count[j] {
                    v_score = 0;
                    break;
                }
                v_score += word_count[j] * score[j];
            }
            if v_score > max {
                max = v_score;
            }
        }
        return max;
    }
}
