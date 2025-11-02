fn main() {
    assert_eq!(Solution::min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4]), 4);
    assert_eq!(
        Solution::min_moves_to_seat(vec![4, 1, 5, 9], vec![1, 3, 2, 6]),
        7
    );
    assert_eq!(
        Solution::min_moves_to_seat(vec![2, 2, 6, 6], vec![1, 3, 2, 6]),
        4
    );
}

struct Solution;
impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        return {
            let mut seats = seats;
            let mut students = students;
            seats.sort_unstable();
            students.sort_unstable();
            seats
                .iter()
                .zip(students.iter())
                .map(|(s, t)| (s - t).abs())
                .sum()
        };
    }
}
