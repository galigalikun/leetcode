fn main() {
    assert_eq!(Solution::max_count(3, 3, vec![vec![2, 2], vec![3, 3]]), 4);
    assert_eq!(
        Solution::max_count(
            3,
            3,
            vec![
                vec![2, 2],
                vec![3, 3],
                vec![3, 3],
                vec![3, 3],
                vec![2, 2],
                vec![3, 3],
                vec![3, 3],
                vec![3, 3],
                vec![2, 2],
                vec![3, 3],
                vec![3, 3],
                vec![3, 3]
            ]
        ),
        4
    );
    assert_eq!(Solution::max_count(3, 3, vec![]), 9);
    assert_eq!(Solution::max_count(4000, 4000, vec![]), 16000000);
    assert_eq!(
        Solution::max_count(39999, 39999, vec![vec![19999, 19999]]),
        399960001
    );
}

struct Solution {}
impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let (mut x, mut y) = (m, n);
        for op in ops {
            x = std::cmp::min(x, op[0]);
            y = std::cmp::min(y, op[1]);
        }

        return x * y;
    }
}
