fn main() {
    assert_eq!(Solution::get_happy_string(1, 3), "c");
    assert_eq!(Solution::get_happy_string(1, 4), "");
    assert_eq!(Solution::get_happy_string(3, 9), "cab");
}

struct Solution;
impl Solution {
    fn dfs(n: i32, k: i32, s: String, res: &mut Vec<String>, count: &mut i32) {
        if s.len() == n as usize {
            *count += 1;
            if *count == k {
                res.push(s);
            }
            return;
        }
        for c in ['a', 'b', 'c'].iter() {
            if s.len() == 0 || s.chars().last().unwrap() != *c {
                Solution::dfs(n, k, s.clone() + &c.to_string(), res, count);
            }
        }
    }
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let mut res = vec![];
        let mut count = 0;
        Solution::dfs(n, k, "".to_string(), &mut res, &mut count);
        if res.len() > 0 {
            return res[0].clone();
        }
        return "".to_string();
    }
}
