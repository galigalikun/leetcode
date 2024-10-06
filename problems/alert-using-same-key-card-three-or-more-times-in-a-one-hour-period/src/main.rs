fn main() {
    assert_eq!(
        Solution::alert_names(
            vec![
                "daniel".to_string(),
                "daniel".to_string(),
                "daniel".to_string(),
                "luis".to_string(),
                "luis".to_string(),
                "luis".to_string(),
                "luis".to_string()
            ],
            vec![
                "10:00".to_string(),
                "10:40".to_string(),
                "11:00".to_string(),
                "09:00".to_string(),
                "11:00".to_string(),
                "13:00".to_string(),
                "15:00".to_string()
            ]
        ),
        vec!["daniel"]
    );
    assert_eq!(
        Solution::alert_names(
            vec![
                "alice".to_string(),
                "alice".to_string(),
                "alice".to_string(),
                "bob".to_string(),
                "bob".to_string(),
                "bob".to_string(),
                "bob".to_string()
            ],
            vec![
                "12:01".to_string(),
                "12:00".to_string(),
                "18:00".to_string(),
                "21:00".to_string(),
                "21:20".to_string(),
                "21:30".to_string(),
                "23:00".to_string()
            ]
        ),
        vec!["bob"]
    );
}

struct Solution;
impl Solution {
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for i in 0..key_name.len() {
            let name = key_name[i].clone();
            let time = key_time[i].clone();
            let time = time.split(":").collect::<Vec<&str>>();
            let time = time[0].parse::<i32>().unwrap() * 60 + time[1].parse::<i32>().unwrap();
            map.entry(name).or_insert(vec![]).push(time);
        }
        let mut res = vec![];
        for (name, times) in map.iter() {
            let mut times = times.clone();
            times.sort();
            let mut left = 0;
            let mut right = 0;
            while right < times.len() {
                if times[right] - times[left] <= 60 {
                    if right - left >= 2 {
                        res.push(name.clone());
                        break;
                    }
                    right += 1;
                } else {
                    left += 1;
                }
            }
        }
        res.sort();
        res
    }
}
