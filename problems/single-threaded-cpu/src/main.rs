fn main() {
    assert_eq!(
        Solution::get_order(vec![vec![1, 2], vec![2, 4], vec![3, 2], vec![4, 1]]),
        vec![0, 2, 3, 1]
    );
    assert_eq!(
        Solution::get_order(vec![
            vec![7, 10],
            vec![7, 12],
            vec![7, 5],
            vec![7, 4],
            vec![7, 2]
        ]),
        vec![4, 3, 2, 0, 1]
    );
}

struct Solution;
impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut tasks: Vec<(i32, i32, i32)> = tasks
            .into_iter()
            .enumerate()
            .map(|(i, task)| (task[0], task[1], i as i32))
            .collect();
        tasks.sort_by(|a, b| a.0.cmp(&b.0));
        let mut result = vec![];
        let mut time = 0;
        let mut i = 0;
        let mut queue = std::collections::BinaryHeap::new();
        while i < tasks.len() || !queue.is_empty() {
            while i < tasks.len() && tasks[i].0 <= time {
                queue.push((tasks[i].1, tasks[i].2));
                i += 1;
            }
            if let Some((_, index)) = queue.pop() {
                result.push(index);
                time += tasks[index as usize].1;
            } else if i < tasks.len() {
                time = tasks[i].0;
            }
        }
        result.sort();
        result
    }
}
