fn main() {
    assert_eq!(
        Solution::find_min_step("WRRBBW".to_string(), "RB".to_string()),
        -1
    );
    assert_eq!(
        Solution::find_min_step("WWRRBBWW".to_string(), "WRBRW".to_string()),
        2
    );
    assert_eq!(
        Solution::find_min_step("G".to_string(), "GGGGG".to_string()),
        2
    );
    assert_eq!(
        Solution::find_min_step("RBYYBBRRB".to_string(), "YRBGB".to_string()),
        3
    );
}

pub struct Solution {}
// https://walkccc.me/LeetCode/problems/0488/
use std::{collections::HashMap, collections::HashSet};
impl Solution {
    fn dfs(board: String, hand: String, map: &mut HashMap<String, i32>) -> i32 {
        let key = format!("{}#{}", board, hand);
        if let Some(&v) = map.get(&key) {
            return v;
        }

        let new_board = Solution::de_dup(board);
        if new_board == "#" {
            return 0;
        }

        let mut board_set = HashSet::new();
        for c in new_board.chars() {
            board_set.insert(c);
        }

        let mut sb = "".to_string();
        for c in hand.chars() {
            if board_set.contains(&c) {
                sb = format!("{}{}", sb, c);
            }
        }

        if sb.len() == 0 {
            return std::i32::MAX;
        }

        let mut ans = std::i32::MAX;
        for i in 0..new_board.len() {
            for j in 0..sb.len() {
                let new_hand = format!("{}{}", &sb[0..j], &sb[j..j + 1]);
                let nn_board = format!(
                    "{}{}{}",
                    &new_board[0..i],
                    sb.chars().nth(j).unwrap(),
                    &new_board[i..]
                );
                let res = Solution::dfs(nn_board, new_hand, map);
                if res < std::i32::MAX {
                    ans = std::cmp::max(ans, 1 + res);
                }
            }
        }

        map.insert(key, ans);
        return ans;
    }
    fn de_dup(board: String) -> String {
        let mut start = 0;
        for i in 0..board.len() {
            if board.chars().nth(i) != board.chars().nth(start) {
                if i - start >= 3 {
                    return Solution::de_dup(format!("{}{}", &board[0..start], &board[i..]));
                }
                start = i;
            }
        }
        return board;
    }
    pub fn find_min_step(board: String, hand: String) -> i32 {
        let mut map = HashMap::new();
        return Solution::dfs(board, hand, &mut map);
    }
}
