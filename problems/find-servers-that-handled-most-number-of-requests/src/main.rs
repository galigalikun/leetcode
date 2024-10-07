use std::vec;

fn main() {
    assert_eq!(
        Solution::busiest_servers(3, vec![1, 2, 3, 4, 5], vec![5, 2, 3, 3, 3]),
        vec![1]
    );
    assert_eq!(
        Solution::busiest_servers(3, vec![1, 2, 3, 4], vec![1, 2, 1, 2]),
        vec![0]
    );
    assert_eq!(
        Solution::busiest_servers(3, vec![1, 2, 3], vec![10, 12, 11]),
        vec![0, 1, 2]
    );
}

struct Solution;
impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        let mut servers = vec![0; k as usize];
        let mut busy_until = vec![0; k as usize];
        let mut max_load = 0;
        let mut max_load_servers = vec![];
        for i in 0..arrival.len() {
            let server = i as i32 % k;
            let mut j = server;
            while j < k + server {
                let idx = (j % k) as usize;
                if arrival[i] >= busy_until[idx] {
                    servers[idx] += 1;
                    busy_until[idx] = arrival[i] + load[i];
                    if servers[idx] > max_load {
                        max_load = servers[idx];
                        max_load_servers = vec![idx as i32];
                    } else if servers[idx] == max_load {
                        max_load_servers.push(idx as i32);
                    }
                    break;
                }
                j += 1;
            }
        }
        max_load_servers
    }
}
