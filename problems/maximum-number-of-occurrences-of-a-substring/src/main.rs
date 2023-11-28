use std::vec;

fn main() {
    assert_eq!(Solution::max_freq("aababcaab".to_string(), 2, 3, 4), 2);
    assert_eq!(Solution::max_freq("aaaa".to_string(), 1, 3, 3), 2);
}

struct Solution;
impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
        let mut max = 0;
        let mut map = std::collections::HashMap::new();
        let mut chars = s.chars();
        let mut window = chars
            .by_ref()
            .take(min_size as usize)
            .collect::<Vec<char>>();
        let mut set = window.iter().collect::<std::collections::HashSet<_>>();
        if set.len() <= max_letters as usize {
            *map.entry(window.iter().collect::<String>()).or_insert(0) += 1;
            max = 1;
        }
        for c in chars {
            window.remove(0);
            window.push(c);
            set = window.iter().collect::<std::collections::HashSet<_>>();
            if set.len() <= max_letters as usize {
                *map.entry(window.iter().collect::<String>()).or_insert(0) += 1;
                max = std::cmp::max(max, *map.get(&window.iter().collect::<String>()).unwrap());
            }
        }
        if max > 0 {
            return max;
        }
        return 0;
    }
}
