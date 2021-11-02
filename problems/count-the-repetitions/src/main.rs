fn main() {
    assert_eq!(
        Solution::get_max_repetitions("acb".to_string(), 4, "ab".to_string(), 2),
        2
    );
    assert_eq!(
        Solution::get_max_repetitions("acb".to_string(), 1, "acb".to_string(), 1),
        1
    );
    assert_eq!(
        Solution::get_max_repetitions("bbaa".to_string(), 2, "b".to_string(), 1),
        5
    );

    assert_eq!(
        Solution::get_max_repetitions("bbaa".to_string(), 2, "b".to_string(), 1),
        4
    );
}

pub struct Solution {}
// https://www.tutorialspoint.com/count-the-repetitions-in-cplusplus
impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let (mut p1, mut p2, mut mark) = (0, 0, 0);
        let s1_len = s1.len() as i32;
        let s2_len = s2.len() as i32;

        for c in s2.chars() {
            if None == s1.chars().find(|&x| x == c) {
                return 0;
            }
        }
        while p1 < s1_len * n1 {
            let c = s2.chars().nth((p2 % s2_len) as usize);
            while s1.chars().nth((p1 % s1_len) as usize) != c && p1 < s1_len * n1 {
                p1 += 1;
            }
            p1 += 1;
            p2 += 1;
            if p2 % s2_len == 0 {
                if p2 == s2_len {
                    mark = p1;
                } else if p1 % s1_len == mark % s1_len {
                    let round = (s1_len * n1 - p1) / (p1 - mark);
                    p1 += round * (p1 - mark);
                    p2 += round * (p2 - s2_len);
                }
            }
        }
        return p2 / s2_len / n2;
    }
}
