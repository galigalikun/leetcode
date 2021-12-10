fn main() {
    assert_eq!(Solution::find_min_difference(vec!["23:59".to_string(),"00:00".to_string()]), 1);
    assert_eq!(Solution::find_min_difference(vec!["00:00".to_string(),"23:59".to_string(),"00:00".to_string()]), 0);
}

struct Solution{}
impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        for time in time_points {
            let mut a = time.split(":");
            let sec = a.nth(0).unwrap().parse::<i32>().unwrap()*60+a.nth(1).unwrap().parse::<i32>().unwrap();
            println!("debug {}", sec);
        }
        return 0;
    }
}
