fn main() {
    assert_eq!(Solution::count_arrangement(2), 2);
    assert_eq!(Solution::count_arrangement(1), 1);
    assert_eq!(Solution::count_arrangement(4), 8);
}

struct Solution {}
// https://www.tutorialspoint.com/beautiful-arrangement-in-cplusplus
impl Solution {
    fn helper(ans: &mut i32, visited: &mut Vec<bool>, end: usize, pos: usize) {
        if pos == end + 1 {
            *ans += 1;
            return;
        }
        for i in 1..=end {
            if !visited[i] && (pos % i == 0 || i % pos == 0) {
                visited[i] = true;
                Solution::helper(ans, visited, end, pos + 1);
                visited[i] = false;
            }
        }
    }
    pub fn count_arrangement(n: i32) -> i32 {
        let mut ans = 0;
        let mut visited = vec![false; n as usize + 1];
        Solution::helper(&mut ans, &mut visited, n as usize, 1);
        return ans;
    }
}
