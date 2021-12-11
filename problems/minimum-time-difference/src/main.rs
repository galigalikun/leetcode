fn main() {
    assert_eq!(
        Solution::find_min_difference(vec!["23:59".to_string(), "00:00".to_string()]),
        1
    );
    assert_eq!(
        Solution::find_min_difference(vec![
            "00:00".to_string(),
            "23:59".to_string(),
            "00:00".to_string()
        ]),
        0
    );
}

struct Solution {}
// https://algorithms.tutorialhorizon.com/minimum-time-difference/
impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut ans = std::i32::MAX;

        let mut lst = vec![];
        for time in time_points {
            let sec = time.split(":").nth(0).unwrap().parse::<i32>().unwrap() * 60
                + time.split(":").nth(1).unwrap().parse::<i32>().unwrap();
            lst.push(sec);
        }

        lst.sort();
        let first = lst.first().unwrap();
        let last = lst.last().unwrap();
        ans = std::cmp::min(ans, first + 1440 - last);
        let mut prev = first;
        for i in 1..lst.len() {
            ans = std::cmp::min(
                ans,
                std::cmp::min((lst[i] - prev).abs(), prev + 1440 - lst[i]).abs(),
            );
            prev = &lst[i];
        }
        return ans;
    }
}
