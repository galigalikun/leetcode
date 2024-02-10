fn main() {
    assert_eq!(Solution::days_between_dates("2019-06-29".to_string(), "2019-06-30".to_string()), 1);
    assert_eq!(Solution::days_between_dates("2020-01-15".to_string(), "2019-12-31".to_string()), 15);
}

struct Solution;
impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        let date1 = date1.parse::<chrono::NaiveDate>().unwrap();
        let date2 = date2.parse::<chrono::NaiveDate>().unwrap();
        let diff = date1.signed_duration_since(date2).num_days();
        return if diff < 0 {
            -diff
        } else {
            diff
        } as i32;
    }
}
