fn main() {
    assert_eq!(
        Solution::max_num_of_substrings("adefaddaccc".to_string()),
        vec!["e", "f", "ccc"]
    );
    assert_eq!(
        Solution::max_num_of_substrings("abbaccd".to_string()),
        vec!["d", "bb", "cc"]
    );
}

struct Solution;
impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        let mut start = vec![26; 26];
        let mut end = vec![0; 26];
        let mut i = 0;
        for c in s.chars() {
            let idx = (c as u8 - 'a' as u8) as usize;
            start[idx] = start[idx].min(i);
            end[idx] = i;
            i += 1;
        }
        let mut res = vec![];
        let mut last = 0;
        for i in 0..26 {
            if start[i] == 26 {
                continue;
            }
            let mut j = end[i];
            let mut ok = true;
            for k in start[i]..=end[i] {
                let idx = (s.chars().nth(k).unwrap() as u8 - 'a' as u8) as usize;
                if start[idx] < start[i] {
                    ok = false;
                    break;
                }
                j = j.max(end[idx]);
            }
            if ok {
                if start[i] > last {
                    res.push(s[start[i]..=j].to_string());
                    last = j;
                }
            }
        }
        return res;
    }
}
