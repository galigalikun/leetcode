fn main() {
    assert_eq!(
        Solution::smallest_chair(vec![vec![1, 4], vec![2, 3], vec![4, 6]], 1),
        1
    );
    assert_eq!(
        Solution::smallest_chair(vec![vec![3, 10], vec![1, 5], vec![2, 6]], 0),
        2
    );
}

struct Solution;
impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        return times
            .iter()
            .enumerate()
            .filter(|(i, _)| *i as i32 != target_friend)
            .map(|(_, time)| time[0])
            .min()
            .unwrap_or(-1);
    }
}
