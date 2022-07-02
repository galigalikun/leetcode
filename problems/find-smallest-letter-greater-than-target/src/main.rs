fn main() {
    assert_eq!(
        Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'a'),
        'c'
    );
    assert_eq!(
        Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'c'),
        'f'
    );
    assert_eq!(
        Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'd'),
        'f'
    );
    assert_eq!(Solution::next_greatest_letter(vec!['c','f','j'], 'j'), 'c');
}

struct Solution {}
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut ans = letters[0];
        for i in letters {
            if i > target {
                ans = std::cmp::min(ans, i);
            } else if i == target {
                ans = std::cmp::min(ans, i);
            }
        }
        return ans;
    }
}
