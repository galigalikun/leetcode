fn main() {
    assert_eq!(
        Solution::intersection_size_two(vec![vec![1, 3], vec![1, 4], vec![2, 5], vec![3, 5]]),
        3
    );
    assert_eq!(
        Solution::intersection_size_two(vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![4, 5]]),
        5
    );
}

struct Solution {}
impl Solution {
    pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
        for (i, v) in intervals.iter().enumerate() {
            println!("{} {} {}", i, v[0], v[1]);
        }
        return -1;
    }
}
