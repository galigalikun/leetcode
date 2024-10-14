fn main() {
    assert_eq!(
        Solution::maximal_network_rank(4, vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![1, 3]]),
        4
    );
    assert_eq!(
        Solution::maximal_network_rank(
            5,
            vec![
                vec![0, 1],
                vec![0, 3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
                vec![2, 4]
            ]
        ),
        5
    );
    assert_eq!(
        Solution::maximal_network_rank(
            8,
            vec![
                vec![0, 1],
                vec![1, 2],
                vec![2, 3],
                vec![2, 4],
                vec![5, 6],
                vec![5, 7]
            ]
        ),
        5
    );
}

struct Solution;
impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut adj = vec![vec![false; n as usize]; n as usize];
        let mut cnt = vec![0; n as usize];
        for road in roads {
            let (a, b) = (road[0] as usize, road[1] as usize);
            adj[a][b] = true;
            adj[b][a] = true;
            cnt[a] += 1;
            cnt[b] += 1;
        }
        let mut ans = 0;
        for i in 0..n {
            for j in i + 1..n {
                let mut rank = cnt[i as usize] + cnt[j as usize];
                if adj[i as usize][j as usize] {
                    rank -= 1;
                }
                ans = ans.max(rank);
            }
        }
        ans
    }
}
