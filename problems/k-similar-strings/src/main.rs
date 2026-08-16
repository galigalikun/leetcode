use std::collections::{HashSet, VecDeque};

fn main() {
    assert_eq!(
        Solution::k_similarity("ab".to_string(), "ba".to_string()),
        1
    );
    assert_eq!(
        Solution::k_similarity("abc".to_string(), "bca".to_string()),
        2
    );
}

struct Solution;
impl Solution {
    pub fn k_similarity(s1: String, s2: String) -> i32 {
        if s1 == s2 {
            return 0;
        }

        let target: Vec<u8> = s2.into_bytes();
        let mut visited: HashSet<Vec<u8>> = HashSet::new();
        let mut queue: VecDeque<(Vec<u8>, i32)> = VecDeque::new();

        let start = s1.into_bytes();
        visited.insert(start.clone());
        queue.push_back((start, 0));

        while let Some((current, steps)) = queue.pop_front() {
            if current == target {
                return steps;
            }

            let mut i = 0;
            while i < current.len() && current[i] == target[i] {
                i += 1;
            }

            for j in (i + 1)..current.len() {
                if current[j] == target[i] && current[j] != target[j] {
                    let mut next = current.clone();
                    next.swap(i, j);

                    if visited.insert(next.clone()) {
                        queue.push_back((next, steps + 1));
                    }
                }
            }
        }

        0
    }
}
