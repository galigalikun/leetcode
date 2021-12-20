fn main() {
    assert_eq!(
        Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]),
        2
    );
    assert_eq!(
        Solution::find_circle_num(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
        3
    );
}

// https://github.com/eMahtab/number-of-provinces
struct Solution {}
impl Solution {
    fn helper(visited: &mut Vec<bool>, is_connected: Vec<Vec<i32>>, i: usize) {
        for j in 0..is_connected.len() {
            if is_connected[i][j] == 1 && !visited[j] {
                visited[j] = true;
                Solution::helper(visited, is_connected.clone(), j);
            }
        }
    }
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![false; is_connected.len()];
        let mut ans = 0;
        for i in 0..is_connected.len() {
            if !visited[i] {
                visited[i] = true;
                ans += 1;
                Solution::helper(&mut visited, is_connected.clone(), i);
            }
        }
        return ans;
    }
}
