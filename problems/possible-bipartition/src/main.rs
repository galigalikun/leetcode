fn main() {
    assert_eq!(
        Solution::possible_bipartition(4, vec![vec![1, 2], vec![1, 3], vec![2, 4]]),
        true
    );
    assert_eq!(
        Solution::possible_bipartition(3, vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
        false
    );
    assert_eq!(
        Solution::possible_bipartition(
            5,
            vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]]
        ),
        false
    );
}

struct Solution;
impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let size = n as usize;
        let mut graph = vec![Vec::new(); size + 1];

        for edge in dislikes {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            graph[a].push(b);
            graph[b].push(a);
        }

        let mut color = vec![0_i8; size + 1];
        let mut queue = std::collections::VecDeque::new();

        for person in 1..=size {
            if color[person] != 0 {
                continue;
            }

            color[person] = 1;
            queue.push_back(person);

            while let Some(current) = queue.pop_front() {
                for &neighbor in &graph[current] {
                    if color[neighbor] == 0 {
                        color[neighbor] = -color[current];
                        queue.push_back(neighbor);
                    } else if color[neighbor] == color[current] {
                        return false;
                    }
                }
            }
        }

        true
    }
}
