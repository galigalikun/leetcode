fn main() {
    assert_eq!(Solution::longest_palindrome_subseq("bbbab".to_string()), 4);
    assert_eq!(Solution::longest_palindrome_subseq("cbbd".to_string()), 2);
    assert_eq!(Solution::longest_palindrome_subseq("aabaa".to_string()), 5);
}

struct Solution {}
// https://www.geeksforgeeks.org/longest-palindromic-subsequence-dp-12/
impl Solution {
    fn helper(s: String, i: usize, j: usize) -> i32 {
        if i == j {
            return 1;
        }
        if s.chars().nth(i) == s.chars().nth(j) && i + 1 == j {
            return 2;
        }
        if s.chars().nth(i) == s.chars().nth(j) {
            return Solution::helper(s, i + 1, j - 1) + 2;
        }

        return std::cmp::max(
            Solution::helper(s.clone(), i, j - 1),
            Solution::helper(s, i + 1, j),
        );
    }
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        return Solution::helper(s.clone(), 0, s.len() - 1);
    }
}
