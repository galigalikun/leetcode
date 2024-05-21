fn main() {
    assert_eq!(Solution::busy_student(vec![1, 2, 3], vec![3, 2, 7], 4), 1);
    assert_eq!(Solution::busy_student(vec![4], vec![4], 4), 1);
}

struct Solution;
impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        return start_time
            .iter()
            .zip(end_time.iter())
            .filter(|(s, e)| *s <= &query_time && *e >= &query_time)
            .count() as i32;
    }
}
