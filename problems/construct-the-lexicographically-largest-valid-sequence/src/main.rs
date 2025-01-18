fn main() {
    assert_eq!(
        Solution::construct_distanced_sequence(3),
        vec![3, 1, 2, 3, 2]
    );
    assert_eq!(
        Solution::construct_distanced_sequence(5),
        vec![5, 3, 1, 4, 3, 5, 2, 4, 2]
    );
}

struct Solution;
impl Solution {
    fn dfs(res: &mut Vec<i32>, visited: &mut Vec<bool>, n: usize, idx: usize) -> bool {
        if idx == res.len() {
            return true;
        }
        if res[idx] != 0 {
            return Self::dfs(res, visited, n, idx + 1);
        }
        for i in (1..=n).rev() {
            if visited[i] {
                continue;
            }
            if i == 1 {
                res[idx] = 1;
                visited[i] = true;
                if Self::dfs(res, visited, n, idx + 1) {
                    return true;
                }
                res[idx] = 0;
                visited[i] = false;
            } else if idx + i < res.len() && res[idx + i] == 0 {
                res[idx] = i as i32;
                res[idx + i] = i as i32;
                visited[i] = true;
                if Self::dfs(res, visited, n, idx + 1) {
                    return true;
                }
                res[idx] = 0;
                res[idx + i] = 0;
                visited[i] = false;
            }
        }
        false
    }
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let mut res = vec![0; 2 * n as usize - 1];
        let mut visited = vec![false; n as usize + 1];
        Self::dfs(&mut res, &mut visited, n as usize, 0);
        res
    }
}
