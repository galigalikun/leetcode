fn main() {
    assert_eq!(Solution::day_of_year(String::from("2019-01-09")), 9);
    assert_eq!(Solution::day_of_year(String::from("2019-02-10")), 41);
}

struct Solution;
impl Solution {
    fn get_days(year: i32, month: i32, day: i32) -> i32 {
        let mut days = 0;
        for i in 2019..year {
            if i % 400 == 0 || (i % 4 == 0 && i % 100 != 0) {
                days += 366;
            } else {
                days += 365;
            }
        }
        for i in 1..month {
            match i {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => days += 31,
                4 | 6 | 9 | 11 => days += 30,
                2 => {
                    if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
                        days += 29;
                    } else {
                        days += 28;
                    }
                }
                _ => {}
            }
        }
        days += day;
        return days;
    }
    pub fn day_of_year(date: String) -> i32 {
        let mut date = date.split('-');
        let year = date.next().unwrap().parse::<i32>().unwrap();
        let month = date.next().unwrap().parse::<i32>().unwrap();
        let day = date.next().unwrap().parse::<i32>().unwrap();
        return Solution::get_days(year, month, day);
    }
}
