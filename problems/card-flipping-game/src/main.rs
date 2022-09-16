fn main() {
    assert_eq!(
        Solution::flipgame(vec![1, 2, 4, 4, 7], vec![1, 3, 4, 1, 3]),
        2
    );
    assert_eq!(Solution::flipgame(vec![1], vec![1]), 0);
    assert_eq!(Solution::flipgame(vec![1], vec![2]), 1);
}

struct Solution {}
impl Solution {
    pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
        let mut ans = 0;
        for (i, v) in fronts.iter().enumerate() {
            if v > &backs[i] {
                ans += 1;
            }
        }
        return ans;
    }
}
