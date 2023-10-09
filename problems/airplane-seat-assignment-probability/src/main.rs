fn main() {
    assert_eq!(Solution::nth_person_gets_nth_seat(1), 1.0);
    assert_eq!(Solution::nth_person_gets_nth_seat(2), 0.5);
}

struct Solution;
impl Solution {
    pub fn nth_person_gets_nth_seat(n: i32) -> f64 {
        return if n == 1 { 1.0 } else { 0.5 };
    }
}
