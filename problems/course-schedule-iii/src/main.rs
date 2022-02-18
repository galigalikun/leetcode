fn main() {
    assert_eq!(
        Solution::schedule_course(vec![
            vec![100, 200],
            vec![200, 1300],
            vec![1000, 1250],
            vec![2000, 3200]
        ]),
        3
    );
    assert_eq!(Solution::schedule_course(vec![vec![1, 2]]), 1);
    assert_eq!(Solution::schedule_course(vec![vec![3, 2], vec![4, 3]]), 0);
}

// https://dev.to/seanpgallivan/solution-course-schedule-iii-48hn
struct Solution {}
use std::collections::BinaryHeap;
impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        let mut c = courses;
        let mut total = 0;
        let mut heap = BinaryHeap::new();
        c.sort_by(|a, b| a[1].cmp(&b[1]));
        for t in c {
            let dur = t[0];
            let end = t[1];
            if dur + total <= end {
                total += dur;
                heap.push(dur);
            } else if heap.len() > 0 && heap.peek() > Some(&dur) {
                total += dur - heap.pop().unwrap();
                heap.push(dur);
            }
        }
        return heap.len() as i32;
    }
}
