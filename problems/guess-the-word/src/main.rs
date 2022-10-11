fn main() {
    let mut master = &Master {};
    master.guess("aaaaaa");
    Solution::find_secret_word(vec!["acckzz", "ccbazz", "eiowzz", "abcczz"], master);

    assert_eq!()
}

struct Solution {}
// This is the Master's API interface.
// You should not implement it, or speculate about its implementation
struct Master;
impl Master {
    fn guess(word: String) -> i32 {
        return 0;
    }
}

impl Solution {
    pub fn find_secret_word(words: Vec<String>, master: &Master) {}
}
