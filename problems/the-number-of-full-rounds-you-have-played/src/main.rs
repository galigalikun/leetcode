fn main() {
    assert_eq!(
        Solution::number_of_rounds("09:31".to_string(), "10:14".to_string()),
        1
    );
    assert_eq!(
        Solution::number_of_rounds("21:30".to_string(), "03:00".to_string()),
        22
    );
}

struct Solution;
impl Solution {
    pub fn number_of_rounds(login_time: String, logout_time: String) -> i32 {
        return if login_time > logout_time {
            Self::number_of_rounds(logout_time, login_time)
        } else {
            let (login_hour, login_minute) = login_time.split_once(':').unwrap();
            let (logout_hour, logout_minute) = logout_time.split_once(':').unwrap();
            let login_hour: i32 = login_hour.parse().unwrap();
            let login_minute: i32 = login_minute.parse().unwrap();
            let logout_hour: i32 = logout_hour.parse().unwrap();
            let logout_minute: i32 = logout_minute.parse().unwrap();

            if logout_hour < login_hour
                || (logout_hour == login_hour && logout_minute < login_minute)
            {
                0
            } else {
                let start_round = (login_minute + 14) / 15;
                let end_round = (logout_minute + 14) / 15;
                (logout_hour - login_hour) * 4 + end_round - start_round
            }
        };
    }
}
