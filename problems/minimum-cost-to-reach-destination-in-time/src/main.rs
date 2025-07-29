fn main() {
    assert_eq!(
        Solution::min_cost(
            30,
            vec![
                vec![0, 1, 10],
                vec![1, 2, 10],
                vec![2, 5, 10],
                vec![0, 3, 1],
                vec![3, 4, 10],
                vec![4, 5, 15]
            ],
            vec![5, 1, 2, 20, 20, 3]
        ),
        11
    );
    assert_eq!(
        Solution::min_cost(
            29,
            vec![
                vec![0, 1, 10],
                vec![1, 2, 10],
                vec![2, 5, 10],
                vec![0, 3, 1],
                vec![3, 4, 10],
                vec![4, 5, 15]
            ],
            vec![5, 1, 2, 20, 20, 3]
        ),
        48
    );
    assert_eq!(
        Solution::min_cost(
            25,
            vec![
                vec![0, 1, 10],
                vec![1, 2, 10],
                vec![2, 5, 10],
                vec![0, 3, 1],
                vec![3, 4, 10],
                vec![4, 5, 15]
            ],
            vec![5, 1, 2, 20, 20, 3]
        ),
        -1
    );
}

struct Solution;
impl Solution {
    pub fn min_cost(max_time: i32, edges: Vec<Vec<i32>>, passing_fees: Vec<i32>) -> i32 {
        let n = passing_fees.len();
        let mut graph = vec![vec![]; n];
        for edge in edges {
            let (u, v, time) = (edge[0] as usize, edge[1] as usize, edge[2]);
            graph[u].push((v, time));
            graph[v].push((u, time));
        }
        let mut dp = vec![vec![i32::MAX; max_time as usize + 1]; n];
        dp[0][0] = passing_fees[0];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, 0)); // (node, time)
        while let Some((node, time)) = queue.pop_front() {
            for &(next_node, travel_time) in &graph[node] {
                let new_time = time + travel_time;
                if new_time <= max_time {
                    let new_fee = dp[node][time as usize] + passing_fees[next_node];
                    if new_fee < dp[next_node][new_time as usize] {
                        dp[next_node][new_time as usize] = new_fee;
                        queue.push_back((next_node, new_time));
                    }
                }
            }
        }
        let mut min_cost = i32::MAX;
        for time in 0..=max_time as usize {
            min_cost = min_cost.min(dp[n - 1][time]);
        }
        if min_cost == i32::MAX {
            return -1;
        }
        min_cost
    }
}
