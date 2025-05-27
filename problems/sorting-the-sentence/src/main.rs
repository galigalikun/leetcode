fn main() {
    assert_eq!(
        Solution::sort_sentence("is2 sentence4 This1 a3".to_string()),
        "This is a sentence"
    );
    assert_eq!(
        Solution::sort_sentence("Myself2 Me1 I4 and3".to_string()),
        "Me Myself and I"
    );
}

struct Solution;
impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut words = s
            .split(' ')
            .map(|word| {
                let mut chars = word.chars();
                let last_char = chars.next_back().unwrap();
                (last_char.to_digit(10).unwrap() as usize, chars.as_str())
            })
            .collect::<Vec<_>>();
        words.sort_by_key(|&(index, _)| index);
        words
            .into_iter()
            .map(|(_, word)| word)
            .collect::<Vec<_>>()
            .join(" ")
    }
}
