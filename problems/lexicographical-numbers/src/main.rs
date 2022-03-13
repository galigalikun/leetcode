fn main() {
    assert_eq!(
        Solution::lexical_order(13),
        vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
    );

    assert_eq!(Solution::lexical_order(2), vec![1, 2]);
}

struct Solution {}
impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut v = (1..=n)
            .map(|x| (format!("{:0<width$}", x, width = n as usize / 10 + 1), x))
            .collect::<Vec<_>>();
        v.sort_by(|a, b| a.cmp(b));
        return v.iter().map(|(_x, y)| *y).collect::<Vec<_>>();
    }
}
