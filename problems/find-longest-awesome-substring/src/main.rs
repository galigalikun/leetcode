fn main() {
    assert_eq!(Solution::longest_awesome("3242415".to_string()), 5);
    assert_eq!(Solution::longest_awesome("12345678".to_string()), 1);
    assert_eq!(Solution::longest_awesome("213123".to_string()), 6);
}

struct Solution;
impl Solution {
    pub fn longest_awesome(s: String) -> i32 {
        let mut mask = 0;
        let mut map = vec![-1; 1024];
        map[0] = 0;
        let mut res = 0;
        for (i, c) in s.chars().enumerate() {
            mask ^= 1 << (c as u8 - b'0');
            if map[mask] != -1 {
                res = res.max(i as i32 + 1 - map[mask]);
            }
            for j in 0..10 {
                if map[mask ^ (1 << j)] != -1 {
                    res = res.max(i as i32 + 1 - map[mask ^ (1 << j)]);
                }
            }
            if map[mask] == -1 {
                map[mask] = i as i32 + 1;
            }
        }
        res
    }
}
