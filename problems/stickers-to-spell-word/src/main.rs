fn main() {
    assert_eq!(
        Solution::min_stickers(
            vec![
                "with".to_string(),
                "example".to_string(),
                "science".to_string()
            ],
            "thehat".to_string()
        ),
        3
    );
    assert_eq!(
        Solution::min_stickers(
            vec!["notice".to_string(), "possible".to_string()],
            "basicbasic".to_string()
        ),
        -1
    );
}

// https://chowdera.com/2022/03/202203200132400249.html
struct Solution {}
impl Solution {
    fn helper(mut status: usize, s: String, target: String) -> usize {
        for c in s.chars() {
            for k in 0..target.len() {
                if ((status >> k) & 1) == 0 && target.chars().nth(k) == Some(c) {
                    let n = 1 << k;
                    status += n;
                    break;
                }
            }
        }
        return status;
    }
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let mut dp = vec![std::i32::MAX; 1 << target.len()];
        dp[0] = 0;
        for i in 0..1 << target.len() {
            if dp[i] == std::i32::MAX {
                continue;
            }
            for sticker in stickers.clone() {
                let j = Solution::helper(i, sticker, target.clone());
                dp[j] = std::cmp::min(dp[j], dp[i] + 1);
            }
        }

        return if dp.last() == Some(&std::i32::MAX) {
            -1
        } else {
            *dp.last().unwrap()
        };
    }
}
