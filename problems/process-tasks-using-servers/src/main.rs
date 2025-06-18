fn main() {
    assert_eq!(
        Solution::assign_tasks(vec![3, 3, 2], vec![1, 2, 3, 2, 1, 2]),
        vec![2, 2, 0, 2, 1, 2]
    );
    assert_eq!(
        Solution::assign_tasks(vec![5, 1, 4, 2, 3], vec![2, 1, 2, 4, 5, 2, 1]),
        vec![1, 4, 1, 4, 1, 3, 2]
    );
}

struct Solution;
impl Solution {
    fn assign_tasks_with_priority_queue(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        use std::collections::BinaryHeap;

        let mut server_heap: BinaryHeap<(i32, i32)> = servers
            .into_iter()
            .enumerate()
            .map(|(i, w)| (-w, i as i32)) // Use negative weight for max-heap
            .collect();

        let mut task_heap: BinaryHeap<(i32, i32)> = tasks
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, t)| (-t, i as i32)) // Use negative time for max-heap
            .collect();

        let mut result = vec![-1; tasks.len()];

        while !task_heap.is_empty() {
            if let Some((_, task_index)) = task_heap.pop() {
                if let Some((neg_server_weight, server_index)) = server_heap.pop() {
                    result[task_index as usize] = server_index;
                    // Reinsert the server with updated weight
                    server_heap.push((neg_server_weight, server_index));
                }
            }
        }

        result
    }
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        return Self::assign_tasks_with_priority_queue(servers, tasks);
    }
}
