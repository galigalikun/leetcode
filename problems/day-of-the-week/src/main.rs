fn main() {
    assert_eq!(Solution::day_of_the_week(31, 8, 2019), "Saturday");
    assert_eq!(Solution::day_of_the_week(18, 7, 1999), "Sunday");
    assert_eq!(Solution::day_of_the_week(15, 8, 1993), "Sunday");
}

struct Solution;
impl Solution {
    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }
    fn days_of_month(year: i32, month: i32) -> i32 {
        match month {
            1 => 31,
            2 => if Solution::is_leap_year(year) { 29 } else { 28 },
            3 => 31,
            4 => 30,
            5 => 31,
            6 => 30,
            7 => 31,
            8 => 31,
            9 => 30,
            10 => 31,
            11 => 30,
            _ => 31,
        }
    }
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let mut days = 0;
        for y in 1971..year {
            days += if Solution::is_leap_year(y) { 366 } else { 365 };
        }
        for m in 1..month {
            days += Solution::days_of_month(year, m);
        }
        days += day - 1;
        let weekday = (days + 5) % 7;
        match weekday {
            0 => "Sunday",
            1 => "Monday",
            2 => "Tuesday",
            3 => "Wednesday",
            4 => "Thursday",
            5 => "Friday",
            _ => "Saturday",
        }.to_string()
    }
}
