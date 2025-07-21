fn main() {
    assert_eq!(Solution::ways_to_build_rooms(vec![-1, 0, 1]), 1);
    assert_eq!(Solution::ways_to_build_rooms(vec![-1, 0, 0, 1, 2]), 6);
}

struct Solution;
impl Solution {
    pub fn ways_to_build_rooms(prev_room: Vec<i32>) -> i32 {
        let n = prev_room.len();
        let mut dp = vec![0; n];
        let mut children = vec![vec![]; n];
        for (i, &p) in prev_room.iter().enumerate() {
            if p != -1 {
                children[p as usize].push(i);
            }
        }
        let mut mod_val = 1_000_000_007;
        let mut dfs = |node: usize| -> i32 {
            let mut ways = 1;
            let mut total_children = 0;
            for &child in &children[node] {
                let child_ways = dfs(child) as i64;
                ways = (ways * child_ways) as i32 % mod_val;
                total_children += child_ways;
            }
            if total_children > 0 {
                ways = (ways * (total_children + 1)) as i32 % mod_val;
            }
            dp[node] = ways;
            ways
        };
        dfs(0);
        dp[0]
    }
}
