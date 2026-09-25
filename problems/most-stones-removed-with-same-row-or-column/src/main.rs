use std::collections::HashMap;

fn main() {
    assert_eq!(Solution::remove_stones(vec![vec![0,0],vec![0,1],vec![1,0],vec![1,2],vec![2,1],vec![2,2]]), 5);
    assert_eq!(Solution::remove_stones(vec![vec![0,0],vec![0,2],vec![1,1],vec![2,0],vec![2,2]]), 3);
    assert_eq!(Solution::remove_stones(vec![vec![0,0]]), 0);
}

struct Solution;
impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let n = stones.len();

        let mut row_to_indices: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut col_to_indices: HashMap<i32, Vec<usize>> = HashMap::new();

        for (index, stone) in stones.iter().enumerate() {
            row_to_indices.entry(stone[0]).or_default().push(index);
            col_to_indices.entry(stone[1]).or_default().push(index);
        }

        let mut visited = vec![false; n];
        let mut components = 0;

        for start in 0..n {
            if visited[start] {
                continue;
            }

            components += 1;
            let mut stack = vec![start];

            while let Some(current) = stack.pop() {
                if visited[current] {
                    continue;
                }

                visited[current] = true;
                let row = stones[current][0];
                let col = stones[current][1];

                if let Some(neighbors) = row_to_indices.remove(&row) {
                    stack.extend(neighbors);
                }

                if let Some(neighbors) = col_to_indices.remove(&col) {
                    stack.extend(neighbors);
                }
            }
        }

        (n - components) as i32
    }
}
