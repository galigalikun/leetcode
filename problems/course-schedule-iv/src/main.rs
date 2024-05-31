fn main() {
    assert_eq!(
        Solution::check_if_prerequisite(2, vec![vec![1, 0]], vec![vec![0, 1], vec![1, 0]]),
        vec![false, true]
    );
    assert_eq!(
        Solution::check_if_prerequisite(2, vec![], vec![vec![1, 0], vec![0, 1]]),
        vec![false, false]
    );
    assert_eq!(
        Solution::check_if_prerequisite(
            3,
            vec![vec![1, 2], vec![1, 0], vec![2, 0]],
            vec![vec![1, 0], vec![1, 2]]
        ),
        vec![true, true]
    );
}

struct Solution;
impl Solution {
    fn dfs(i: usize, graph: &Vec<Vec<usize>>, dp: &mut Vec<Vec<bool>>) {
        for &j in graph[i].iter() {
            if dp[i][j] {
                continue;
            }
            dp[i][j] = true;
            Solution::dfs(j, graph, dp);
            for k in 0..dp.len() {
                if dp[j][k] {
                    dp[i][k] = true;
                }
            }
        }
    }
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        if num_courses == 0 {
            return vec![];
        }
        let mut graph = vec![vec![]; num_courses as usize];
        for pre in prerequisites.iter() {
            graph[pre[0] as usize].push(pre[1] as usize);
        }
        let mut dp = vec![vec![false; num_courses as usize]; num_courses as usize];
        for i in 0..num_courses as usize {
            Solution::dfs(i, &graph, &mut dp);
        }
        let mut res = vec![];
        for q in queries.iter() {
            res.push(dp[q[0] as usize][q[1] as usize]);
        }
        return res;
    }
}
