fn main() {
    assert_eq!(Solution::most_booked(2, vec![vec![0,10],vec![1,5],vec![2,7],vec![3,4]]), 0);
    assert_eq!(Solution::most_booked(3, vec![vec![1,20],vec![2,10],vec![3,5],vec![4,9],vec![6,8]]), 1);
}

struct Solution;
impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut rooms = vec![0; n];
        let mut schedule = vec![vec![]; n];
        let mut meetings = meetings;
        meetings.sort_by_key(|m| m[0]);
        for meeting in meetings {
            let mut idx = 0;
            for i in 1..n {
                if rooms[i] < rooms[idx] || (rooms[i] == rooms[idx] && i < idx) {
                    idx = i;
                }
            }
            if rooms[idx] <= meeting[0] {
                rooms[idx] = meeting[1];
            } else {
                let duration = meeting[1] - meeting[0];
                rooms[idx] += duration;
            }
            schedule[idx].push(meeting);
        }
        let mut ans = 0;
        for i in 1..n {
            if schedule[i].len() > schedule[ans].len() {
                ans = i;
            }
        }
        ans as i32
    }
}
