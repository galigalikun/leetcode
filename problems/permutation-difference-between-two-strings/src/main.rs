fn main() {
    assert_eq!(
        Solution::find_permutation_difference("abc".to_string(), "bac".to_string()),
        2
    );
    assert_eq!(
        Solution::find_permutation_difference("abcde".to_string(), "edbac".to_string()),
        12
    );
}

struct Solution;
impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let n = s.len();
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        let mut count = vec![0; 26];
        let mut result = 0;
        for i in 0..n {
            let s_index = (s_bytes[i] - b'a') as usize;
            let t_index = (t_bytes[i] - b'a') as usize;
            count[s_index] += 1;
            count[t_index] -= 1;

            for j in 0..26 {
                if count[j] > 0 {
                    result += count[j];
                }
            }
        }

        return result;
    }
}
