fn main() {
    assert_eq!(Solution::is_alien_sorted(vec!["hello".to_string(),"leetcode".to_string()], "hlabcdefgijkmnopqrstuvwxyz".to_string()), true);
    assert_eq!(Solution::is_alien_sorted(vec!["word".to_string(),"world".to_string(),"row".to_string()], "worldabcefghijkmnpqstuvxyz".to_string()), false);
    assert_eq!(Solution::is_alien_sorted(vec!["apple".to_string(),"app".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

struct Solution;
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut rank = [0usize; 26];
        for (idx, ch) in order.bytes().enumerate() {
            rank[(ch - b'a') as usize] = idx;
        }

        for pair in words.windows(2) {
            let left = pair[0].as_bytes();
            let right = pair[1].as_bytes();
            let mut differs = false;

            for (&l, &r) in left.iter().zip(right.iter()) {
                let l_rank = rank[(l - b'a') as usize];
                let r_rank = rank[(r - b'a') as usize];

                if l_rank < r_rank {
                    differs = true;
                    break;
                }

                if l_rank > r_rank {
                    return false;
                }
            }

            if !differs && left.len() > right.len() {
                return false;
            }
        }

        true
    }
}
