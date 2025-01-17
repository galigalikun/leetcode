fn main() {
    assert_eq!(Solution::total_money(4), 10);
    assert_eq!(Solution::total_money(10), 37);
    assert_eq!(Solution::total_money(20), 96);
}

struct Solution;
impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut total = 0;
        let mut week = 0;
        let mut day = 0;
        for i in 0..n {
            if i % 7 == 0 && i != 0 {
                week += 1;
                day = 0;
            }
            day += 1;
            total += week + day;
        }
        total
    }
}
