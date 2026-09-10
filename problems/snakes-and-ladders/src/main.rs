use std::collections::VecDeque;

fn main() {
    assert_eq!(Solution::snakes_and_ladders(vec![vec![-1,-1,-1,-1,-1,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,35,-1,-1,13,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,15,-1,-1,-1,-1]]), 4);
    assert_eq!(Solution::snakes_and_ladders(vec![vec![-1,-1],vec![-1,3]]), 1);
}

struct Solution;
impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        if n == 0 {
            return -1;
        }

        let last = n * n;
        if last == 1 {
            return 0;
        }

        let mut square_to_value = vec![-1; last + 1];
        let mut label = 1usize;

        for row in (0..n).rev() {
            let row_from_bottom = n - 1 - row;
            let cols: Vec<_> = if row_from_bottom % 2 == 0 {
                (0..n).collect()
            } else {
                (0..n).rev().collect()
            };

            for col in cols {
                square_to_value[label] = board[row][col];
                label += 1;
            }
        }

        let mut dist = vec![i32::MAX; last + 1];
        let mut queue = VecDeque::new();
        dist[1] = 0;
        queue.push_back(1usize);

        while let Some(curr) = queue.pop_front() {
            for roll in 1..=6 {
                let next = curr + roll;
                if next > last {
                    break;
                }

                let mut dest = next;
                if square_to_value[dest] != -1 {
                    dest = square_to_value[dest] as usize;
                }

                if dist[dest] == i32::MAX {
                    dist[dest] = dist[curr] + 1;
                    if dest == last {
                        return dist[dest];
                    }
                    queue.push_back(dest);
                }
            }
        }

        -1
    }
}
