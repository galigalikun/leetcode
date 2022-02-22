fn main() {
    assert_eq!(
        Solution::exclusive_time(
            2,
            vec![
                "0:start:0".to_string(),
                "1:start:2".to_string(),
                "1:end:5".to_string(),
                "0:end:6".to_string()
            ]
        ),
        vec![3, 4]
    );
    assert_eq!(
        Solution::exclusive_time(
            1,
            vec![
                "0:start:0".to_string(),
                "0:start:2".to_string(),
                "0:end:5".to_string(),
                "0:start:6".to_string(),
                "0:end:6".to_string(),
                "0:end:7".to_string()
            ]
        ),
        vec![8]
    );
    assert_eq!(
        Solution::exclusive_time(
            2,
            vec![
                "0:start:0".to_string(),
                "0:start:2".to_string(),
                "0:end:5".to_string(),
                "1:start:6".to_string(),
                "1:end:6".to_string(),
                "0:end:7".to_string()
            ]
        ),
        vec![7, 1]
    );
}

// https://cheonhyangzhang.gitbooks.io/leetcode-solutions/content/636-exclusive-time-of-functions.html
struct Solution {}
impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut res = vec![0; n as usize];
        let mut jobs = vec![];
        let mut times = vec![];
        for i in 0..logs.len() {
            let mut log = logs[i].split(":");
            let function_id = log.next().unwrap().parse::<i32>().unwrap();
            let op = log.next().unwrap();
            let timestamp = log.next().unwrap().parse::<i32>().unwrap();
            if op == "start" {
                if !jobs.is_empty() {
                    res[*jobs.last().unwrap() as usize] += timestamp - times.last().unwrap();
                }
                jobs.push(function_id);
                times.push(timestamp);
            } else {
                res[function_id as usize] += timestamp - times.last().unwrap() + 1;
                jobs.pop();
                times.pop();
                if !times.is_empty() {
                    times.pop();
                    times.push(timestamp + 1);
                }
            }
        }
        return res;
    }
}
