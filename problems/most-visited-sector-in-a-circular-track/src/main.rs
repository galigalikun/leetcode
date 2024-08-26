fn main() {
    assert_eq!(Solution::most_visited(4, vec![1, 3, 1, 2]), vec![1, 2]);
    assert_eq!(
        Solution::most_visited(2, vec![2, 1, 2, 1, 2, 1, 2, 1, 2]),
        vec![2]
    );
    assert_eq!(
        Solution::most_visited(7, vec![1, 3, 5, 7]),
        vec![1, 2, 3, 4, 5, 6, 7]
    );
}

struct Solution;
impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        let start = rounds[0];
        let end = rounds[rounds.len() - 1];
        if start <= end {
            for i in start..=end {
                res.push(i);
            }
        } else {
            for i in 1..=end {
                res.push(i);
            }
            for i in start..=n {
                res.push(i);
            }
        }
        res
    }
}
