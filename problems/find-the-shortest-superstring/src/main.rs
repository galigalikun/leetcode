fn main() {
    let words1 = vec!["alex".to_string(), "loves".to_string(), "leetcode".to_string()];
    let result1 = Solution::shortest_superstring(words1.clone());
    assert!(words1.iter().all(|w| result1.contains(w)));
    assert_eq!(result1.len(), 17);

    let words2 = vec![
        "catg".to_string(),
        "ctaagt".to_string(),
        "gcta".to_string(),
        "ttca".to_string(),
        "atgcatc".to_string(),
    ];
    let result2 = Solution::shortest_superstring(words2.clone());
    assert!(words2.iter().all(|w| result2.contains(w)));
    assert_eq!(result2.len(), 16);
}

struct Solution;
impl Solution {
    pub fn shortest_superstring(words: Vec<String>) -> String {
        let n = words.len();
        if n == 0 {
            return String::new();
        }

        let mut overlap = vec![vec![0usize; n]; n];
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    overlap[i][j] = Self::calc_overlap(&words[i], &words[j]);
                }
            }
        }

        let max_mask = 1usize << n;
        let mut dp = vec![vec![-1isize; n]; max_mask];
        let mut parent = vec![vec![-1isize; n]; max_mask];

        for i in 0..n {
            dp[1usize << i][i] = 0;
        }

        for mask in 1usize..max_mask {
            for last in 0..n {
                if (mask & (1usize << last)) == 0 {
                    continue;
                }

                let prev_mask = mask ^ (1usize << last);
                if prev_mask == 0 {
                    continue;
                }

                for prev in 0..n {
                    if (prev_mask & (1usize << prev)) == 0 {
                        continue;
                    }
                    if dp[prev_mask][prev] < 0 {
                        continue;
                    }

                    let candidate = dp[prev_mask][prev] + overlap[prev][last] as isize;
                    if candidate > dp[mask][last] {
                        dp[mask][last] = candidate;
                        parent[mask][last] = prev as isize;
                    }
                }
            }
        }

        let full_mask = max_mask - 1;
        let mut end = 0usize;
        for i in 1..n {
            if dp[full_mask][i] > dp[full_mask][end] {
                end = i;
            }
        }

        let mut path = Vec::with_capacity(n);
        let mut mask = full_mask;
        let mut current = end as isize;

        while current >= 0 {
            let idx = current as usize;
            path.push(idx);
            let prev = parent[mask][idx];
            mask ^= 1usize << idx;
            current = prev;
        }
        path.reverse();

        let mut answer = words[path[0]].clone();
        for i in 1..path.len() {
            let prev = path[i - 1];
            let next = path[i];
            let o = overlap[prev][next];
            answer.push_str(&words[next][o..]);
        }

        answer
    }

    fn calc_overlap(a: &str, b: &str) -> usize {
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();
        let max_len = a.len().min(b.len());
        for k in (1..=max_len).rev() {
            if a_bytes[a.len() - k..] == b_bytes[..k] {
                return k;
            }
        }
        0
    }
}
