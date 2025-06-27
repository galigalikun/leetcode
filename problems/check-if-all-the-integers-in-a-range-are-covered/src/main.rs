fn main() {
    assert_eq!(
        Solution::is_covered(vec![vec![1, 2], vec![3, 4], vec![5, 6]], 2, 5),
        true
    );
    assert_eq!(
        Solution::is_covered(vec![vec![1, 10], vec![10, 20]], 21, 21),
        false
    );
}

struct Solution;
impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        return ranges
            .iter()
            .filter(|range| range[0] <= right && range[1] >= left)
            .map(|range| (range[0], range[1]))
            .fold(
                vec![false; (right - left + 1) as usize],
                |mut acc, (start, end)| {
                    for i in start.max(left)..=end.min(right) {
                        acc[(i - left) as usize] = true;
                    }
                    acc
                },
            )
            .iter()
            .all(|&covered| covered);
    }
}
