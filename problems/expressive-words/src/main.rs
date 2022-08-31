fn main() {
    assert_eq!(Solution::expressive_words("heeellooo".to_string(), vec!["hello".to_string(), "hi".to_string(), "helo".to_string()]), 1);
    assert_eq!(Solution::expressive_words("zzzzzyyyyy".to_string(), vec!["zzyy".to_string(),"zy".to_string(),"zyy".to_string()]), 3);
}

struct Solution{}
impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        return 0;
    }
}
