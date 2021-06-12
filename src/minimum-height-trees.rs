fn main() {
    assert_eq!(
        Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]]),
        vec![1]
    );

    assert_eq!(
        Solution::find_min_height_trees(
            6,
            vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]]
        ),
        vec![3, 4]
    );

    assert_eq!(Solution::find_min_height_trees(1, vec![]), vec![0]);

    assert_eq!(
        Solution::find_min_height_trees(2, vec![vec![0, 1]]),
        vec![0, 1]
    );
}

pub struct Solution {}
// https://www.geeksforgeeks.org/roots-tree-gives-minimum-height/
use std::collections::HashMap;
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if edges.len() == 0 {
            return vec![0];
        }
        let mut degree = vec![0; n as usize];
        let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();
        for edge in edges {
            if let Some(a) = adj.get_mut(&edge[0]) {
                (*a).push(edge[1]);
            } else {
                adj.insert(edge[0], vec![edge[1]]);
            }
            if let Some(a) = adj.get_mut(&edge[1]) {
                (*a).push(edge[0]);
            } else {
                adj.insert(edge[1], vec![edge[0]]);
            }
            degree[edge[0] as usize] += 1;
            degree[edge[1] as usize] += 1;
        }

        let mut queue = vec![];
        for i in 0..n {
            if degree[i as usize] == 1 {
                queue.push(i);
            }
        }

        let mut v = n;
        while v > 2 {
            v -= queue.len() as i32;
            for _i in 0..queue.len() {
                // let t = *queue.first().unwrap() as i32;
                let t = queue.remove(0) as i32;
                for &j in adj.get(&t).unwrap() {
                    degree[j as usize] -= 1;
                    if degree[j as usize] == 1 {
                        queue.push(j);
                    }
                }
            }
        }
        return queue;
        // let mut result = vec![];
        // println!("debug queue {:?}", queue);
        // while !queue.is_empty() {
        //     result.push(queue.remove(0) as i32);
        // }
        // println!("debug {:?}", result);
        // return result;
    }
}
