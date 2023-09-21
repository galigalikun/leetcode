fn main() {
    assert_eq!(
        Solution::sort_items(
            8,
            2,
            vec![-1, -1, 1, 0, 0, 1, 0, -1],
            vec![
                vec![],
                vec![6],
                vec![5],
                vec![6],
                vec![3, 6],
                vec![],
                vec![],
                vec![]
            ]
        ),
        vec![6, 3, 4, 1, 5, 2, 0, 7]
    );
    assert_eq!(
        Solution::sort_items(
            8,
            2,
            vec![-1, -1, 1, 0, 0, 1, 0, -1],
            vec![
                vec![],
                vec![6],
                vec![5],
                vec![6],
                vec![3],
                vec![],
                vec![4],
                vec![]
            ]
        ),
        vec![]
    );
}

struct Solution;
impl Solution {
    pub fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
        let mut group_items = vec![vec![]; m as usize];
        let mut group_graph = vec![vec![]; m as usize];
        let mut group_id = m;
        let mut group = group;
        for i in 0..n as usize {
            if group[i] == -1 {
                group[i] = group_id;
                group_id += 1;
            }
            group_items[group[i] as usize].push(i);
        }
        let mut group_indegree = vec![0; group_id as usize];
        let mut item_indegree = vec![0; n as usize];
        for i in 0..n as usize {
            let cur_group = group[i] as usize;
            for &item in &before_items[i] {
                let before_group = group[item as usize] as usize;
                if before_group != cur_group {
                    group_graph[before_group].push(cur_group);
                    group_indegree[cur_group] += 1;
                }
            }
        }
        let mut group_queue = std::collections::VecDeque::new();
        for i in 0..group_id as usize {
            if group_indegree[i] == 0 {
                group_queue.push_back(i);
            }
        }
        let mut res = vec![];
        while !group_queue.is_empty() {
            let cur_group = group_queue.pop_front().unwrap();
            let mut item_queue = std::collections::VecDeque::new();
            for &item in &group_items[cur_group] {
                if item_indegree[item] == 0 {
                    item_queue.push_back(item);
                }
            }
            while !item_queue.is_empty() {
                let cur_item = item_queue.pop_front().unwrap();
                res.push(cur_item as i32);
                for &item in &before_items[cur_item as usize] {
                    let before_group = group[item as usize] as usize;
                    if before_group == cur_group {
                        item_indegree[item as usize] -= 1;
                        if item_indegree[item as usize] == 0 {
                            item_queue.push_back(item as usize);
                        }
                    }
                }
            }
            for &next_group in &group_graph[cur_group] {
                group_indegree[next_group] -= 1;
                if group_indegree[next_group] == 0 {
                    group_queue.push_back(next_group);
                }
            }
        }
        if res.len() == n as usize {
            return res;
        }

        return vec![];
    }
}
