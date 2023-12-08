fn main() {
    assert_eq!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5), true);
    assert_eq!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 0), true);
    assert_eq!(Solution::can_reach(vec![3, 0, 2, 1, 2], 2), false);
}

struct Solution;
impl Solution {
    fn dfs(arr: &Vec<i32>, start: usize, visited: &mut Vec<bool>) -> bool {
        if arr[start] == 0 {
            return true;
        }
        visited[start] = true;
        let mut left = false;
        let mut right = false;
        if start as i32 - arr[start] >= 0 && !visited[(start as i32 - arr[start]) as usize] {
            left = Solution::dfs(arr, (start as i32 - arr[start]) as usize, visited);
        }
        if (start + arr[start] as usize) < arr.len() && !visited[start + arr[start] as usize] {
            right = Solution::dfs(arr, start + arr[start] as usize, visited);
        }
        return left || right;
    }
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut visited = vec![false; arr.len()];
        return Solution::dfs(&arr, start as usize, &mut visited);
    }
}
