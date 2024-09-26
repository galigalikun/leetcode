fn main() {
    assert_eq!(Solution::max_unique_split("ababccc".to_string()), 5);
    assert_eq!(Solution::max_unique_split("aba".to_string()), 2);
    assert_eq!(Solution::max_unique_split("aa".to_string()), 1);
}

struct Solution;
impl Solution {
    fn dfs(s: &mut Vec<char>, set: &mut std::collections::HashSet<String>, max: &mut i32) {
        if s.is_empty() {
            *max = std::cmp::max(*max, set.len() as i32);
            return;
        }
        for i in 1..=s.len() {
            let left = s.drain(0..i).collect::<String>();
            if set.insert(left.clone()) {
                Self::dfs(s, set, max);
                set.remove(&left);
            }
            s.insert(0, left.chars().next().unwrap());
        }
    }
    pub fn max_unique_split(s: String) -> i32 {
        let mut max = 0;
        let mut set = std::collections::HashSet::new();
        let mut s = s.chars().collect::<Vec<_>>();
        Self::dfs(&mut s, &mut set, &mut max);
        max
    }
}
