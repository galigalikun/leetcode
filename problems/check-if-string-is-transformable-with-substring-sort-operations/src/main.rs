fn main() {
    assert_eq!(
        Solution::is_transformable("84532".to_string(), "34852".to_string()),
        true
    );
    assert_eq!(
        Solution::is_transformable("34521".to_string(), "23415".to_string()),
        true
    );
    assert_eq!(
        Solution::is_transformable("12345".to_string(), "12435".to_string()),
        false
    );
    assert_eq!(
        Solution::is_transformable("4941".to_string(), "1494".to_string()),
        true
    );
}

struct Solution;
impl Solution {
    pub fn is_transformable(s: String, t: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let t = t.chars().collect::<Vec<_>>();
        let mut idx = vec![vec![]; 10];
        for i in 0..s.len() {
            idx[s[i] as usize - '0' as usize].push(i);
        }
        for i in 0..t.len() {
            let c = t[i] as usize - '0' as usize;
            if idx[c].is_empty() {
                return false;
            }
            let j = idx[c].last().unwrap();
            for k in 0..c {
                if !idx[k].is_empty() && idx[k][0] < *j {
                    return false;
                }
            }
            idx[c].pop();
        }
        return true;
    }
}
