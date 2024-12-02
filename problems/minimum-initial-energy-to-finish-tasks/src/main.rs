use std::vec;

fn main() {
    assert_eq!(
        Solution::minimum_effort(vec![vec![1, 2], vec![2, 4], vec![4, 8]]),
        8
    );
    assert_eq!(
        Solution::minimum_effort(vec![
            vec![1, 3],
            vec![2, 4],
            vec![10, 11],
            vec![10, 12],
            vec![8, 9]
        ]),
        32
    );
    assert_eq!(
        Solution::minimum_effort(vec![
            vec![1, 7],
            vec![2, 8],
            vec![3, 9],
            vec![4, 10],
            vec![5, 11],
            vec![6, 12]
        ]),
        27
    );
}

struct Solution;
impl Solution {
    pub fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
        let mut tasks = tasks;
        for task in tasks.iter_mut() {
            task.push(task[1] - task[0]);
        }
        tasks.sort_by(|a, b| b[2].cmp(&a[2]));
        let mut energy = 0;
        for task in tasks.iter() {
            energy = energy.max(task[1]) + task[0];
        }
        energy
    }
}
