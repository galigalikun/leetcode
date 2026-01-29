fn main() {
    assert_eq!(
        Solution::count_mentions(
            2,
            vec![
                ["MESSAGE", "10", "id1 id0"],
                ["OFFLINE", "11", "0"],
                ["MESSAGE", "71", "HERE"]
            ]
            .iter()
            .map(|v| v.iter().map(|s| s.to_string()).collect())
            .collect()
        ),
        vec![2, 2]
    );
    assert_eq!(
        Solution::count_mentions(
            2,
            vec![
                ["MESSAGE", "10", "id1 id0"],
                ["OFFLINE", "11", "0"],
                ["MESSAGE", "12", "ALL"]
            ]
            .iter()
            .map(|v| v.iter().map(|s| s.to_string()).collect())
            .collect()
        ),
        vec![2, 2]
    );
    assert_eq!(
        Solution::count_mentions(
            3,
            vec![["OFFLINE", "10", "0"], ["MESSAGE", "12", "HERE"]]
                .iter()
                .map(|v| v.iter().map(|s| s.to_string()).collect())
                .collect()
        ),
        vec![0, 1]
    );
}

struct Solution;
impl Solution {
    pub fn count_mentions(number_of_users: i32, events: Vec<Vec<String>>) -> Vec<i32> {
        let mut results = vec![];
        let mut online_users = number_of_users;
        for event in events {
            match event[0].as_str() {
                "MESSAGE" => {
                    let content = &event[2];
                    if content == "ALL" {
                        results.push(online_users);
                    } else {
                        let mentions: Vec<&str> = content.split_whitespace().collect();
                        results.push(mentions.len() as i32);
                    }
                }
                "OFFLINE" => {
                    online_users -= 1;
                }
                _ => {}
            }
        }
        results
    }
}
