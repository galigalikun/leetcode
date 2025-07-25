fn main() {
    assert_eq!(
        Solution::longest_common_subpath(
            5,
            vec![vec![0, 1, 2, 3, 4], vec![2, 3, 4], vec![4, 0, 1, 2, 3],]
        ),
        2
    );
    assert_eq!(
        Solution::longest_common_subpath(3, vec![vec![0], vec![1], vec![2],]),
        0
    );
    assert_eq!(
        Solution::longest_common_subpath(5, vec![vec![0, 1, 2, 3, 4], vec![4, 3, 2, 1, 0],]),
        1
    );
}

struct Solution;
impl Solution {
    fn has_common_subpath_of_length(length: i32, paths: &[Vec<i32>]) -> bool {
        use std::collections::HashSet;

        let mut seen: HashSet<u64> = HashSet::new();
        let base: u64 = 1_000_000_007;
        let mut hash = 0;
        let mut base_pow = 1;

        for path in paths {
            if path.len() < length as usize {
                continue;
            }

            hash = 0;
            base_pow = 1;

            for i in 0..length as usize {
                hash = (hash * base + path[i] as u64) % u64::MAX;
                if i < length as usize - 1 {
                    base_pow = (base_pow * base) % u64::MAX;
                }
            }

            seen.insert(hash);

            for i in length as usize..path.len() {
                hash = (hash * base + path[i] as u64 - path[i - length as usize] as u64 * base_pow)
                    % u64::MAX;
                seen.insert(hash);
            }
        }

        !seen.is_empty()
    }
    pub fn longest_common_subpath(n: i32, paths: Vec<Vec<i32>>) -> i32 {
        if paths.is_empty() || n == 0 {
            return 0;
        }

        let mut min_length = 1;
        let mut max_length = paths.iter().map(|p| p.len()).min().unwrap_or(0) as i32;

        while min_length <= max_length {
            let mid_length = (min_length + max_length) / 2;
            if Self::has_common_subpath_of_length(mid_length, &paths) {
                min_length = mid_length + 1;
            } else {
                max_length = mid_length - 1;
            }
        }

        max_length
    }
}
