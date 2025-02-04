fn main() {
    assert_eq!(Solution::maximum_time("2?:?0".to_string()), "23:50".to_string());
    assert_eq!(Solution::maximum_time("0?:3?".to_string()), "09:39".to_string());
    assert_eq!(Solution::maximum_time("1?:22".to_string()), "19:22".to_string());
}

struct Solution;
impl Solution {
    pub fn maximum_time(time: String) -> String {
        let mut time = time.chars().collect::<Vec<char>>();
        time[0] = match time[0] {
            '?' => if time[1] == '?' || time[1] < '4' { '2' } else { '1' },
            _ => time[0],
        };
        time[1] = match time[1] {
            '?' => if time[0] == '2' { '3' } else { '9' },
            _ => time[1],
        };
        time[3] = match time[3] {
            '?' => '5',
            _ => time[3],
        };
        time[4] = match time[4] {
            '?' => '9',
            _ => time[4],
        };
        time.iter().collect()
    }
}
