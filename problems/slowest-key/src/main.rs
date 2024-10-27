fn main() {
    assert_eq!(
        Solution::slowest_key(vec![9, 29, 49, 50], "cbcd".to_string()),
        'c'
    );
    assert_eq!(
        Solution::slowest_key(vec![12, 23, 36, 46, 62], "spuda".to_string()),
        'a'
    );
}

struct Solution;
impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut max_duration = release_times[0];
        let mut max_key = keys_pressed.chars().nth(0).unwrap();
        for a in 1..release_times.len() {
            let duration = release_times[a] - release_times[a - 1];
            if duration > max_duration
                || (duration == max_duration && keys_pressed.chars().nth(a).unwrap() > max_key)
            {
                max_duration = duration;
                max_key = keys_pressed.chars().nth(a).unwrap();
            }
        }
        max_key
    }
}
