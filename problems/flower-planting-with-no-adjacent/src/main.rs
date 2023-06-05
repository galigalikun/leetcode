fn main() {
    assert_eq!(
        Solution::garden_no_adj(3, vec![vec![1, 2], vec![2, 3], vec![3, 1]]),
        vec![1, 2, 3]
    );
    assert_eq!(
        Solution::garden_no_adj(4, vec![vec![1, 2], vec![3, 4]]),
        vec![1, 2, 1, 2]
    );
    assert_eq!(
        Solution::garden_no_adj(
            4,
            vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 1],
                vec![1, 3],
                vec![2, 4]
            ]
        ),
        vec![1, 2, 3, 4]
    );
}

struct Solution;
impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![0; n as usize];
        let mut graph = vec![vec![]; n as usize];
        for path in paths {
            graph[(path[0] - 1) as usize].push(path[1] - 1);
            graph[(path[1] - 1) as usize].push(path[0] - 1);
        }
        for i in 0..n as usize {
            let mut colors = vec![0; 5];
            for j in 0..graph[i].len() {
                colors[result[graph[i][j] as usize] as usize] = 1;
            }
            for j in 1..5 {
                if colors[j] == 0 {
                    result[i] = j as i32;
                    break;
                }
            }
        }
        return result;
    }
}
