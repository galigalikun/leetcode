fn main() {
    assert_eq!(
        Solution::min_push_box(vec![
            vec!['#', '#', '#', '#', '#', '#'],
            vec!['#', 'T', '#', '#', '#', '#'],
            vec!['#', '.', '.', 'B', '.', '#'],
            vec!['#', '.', '#', '#', '.', '#'],
            vec!['#', '.', '.', '.', 'S', '#'],
            vec!['#', '#', '#', '#', '#', '#']
        ]),
        3
    );
    assert_eq!(
        Solution::min_push_box(vec![
            vec!['#', '#', '#', '#', '#', '#'],
            vec!['#', 'T', '#', '#', '#', '#'],
            vec!['#', '.', '.', 'B', '.', '#'],
            vec!['#', '#', '#', '#', '.', '#'],
            vec!['#', '.', '.', '.', 'S', '#'],
            vec!['#', '#', '#', '#', '#', '#']
        ]),
        -1
    );
    assert_eq!(
        Solution::min_push_box(vec![
            vec!['#', '#', '#', '#', '#', '#'],
            vec!['#', 'T', '.', '.', '#', '#'],
            vec!['#', '.', '#', 'B', '.', '#'],
            vec!['#', '.', '.', '.', '.', '#'],
            vec!['#', '.', '.', '.', 'S', '#'],
            vec!['#', '#', '#', '#', '#', '#']
        ]),
        5
    );
}

struct Solution;
impl Solution {
    pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
        let mut queue = std::collections::VecDeque::new();
        let mut visited = std::collections::HashSet::new();
        let mut steps = 0;
        let mut player = (0, 0);
        let mut box_ = (0, 0);
        let mut target = (0, 0);
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                match grid[i][j] {
                    'S' => player = (i as i32, j as i32),
                    'B' => box_ = (i as i32, j as i32),
                    'T' => target = (i as i32, j as i32),
                    _ => {}
                }
            }
        }
        queue.push_back((player, box_));
        visited.insert((player, box_));
        while !queue.is_empty() {
            let len = queue.len();
            for _ in 0..len {
                let (player, box_) = queue.pop_front().unwrap();
                if box_ == target {
                    return steps;
                }
                for (i, j) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let (player, box_) = ((player.0 + i, player.1 + j), box_);
                    if player.0 < 0
                        || player.0 >= grid.len() as i32
                        || player.1 < 0
                        || player.1 >= grid[0].len() as i32
                        || grid[player.0 as usize][player.1 as usize] == '#'
                    {
                        continue;
                    }
                    if visited.contains(&(player, box_)) {
                        continue;
                    }
                    let (player, box_) = ((player.0 - i, player.1 - j), box_);
                    if box_ == target {
                        continue;
                    }
                    let (player, box_) = ((player.0 + i, player.1 + j), (box_.0 + i, box_.1 + j));
                    if box_.0 < 0
                        || box_.0 >= grid.len() as i32
                        || box_.1 < 0
                        || box_.1 >= grid[0].len() as i32
                        || grid[box_.0 as usize][box_.1 as usize] == '#'
                    {
                        continue;
                    }
                    if visited.contains(&(player, box_)) {
                        continue;
                    }
                    let (player, box_) = ((player.0 - i, player.1 - j), (box_.0 - i, box_.1 - j));
                    if grid[player.0 as usize][player.1 as usize] == '#'
                        || grid[box_.0 as usize][box_.1 as usize] == '#'
                    {
                        continue;
                    }
                    queue.push_back((player, box_));
                    visited.insert((player, box_));
                }
            }
            steps += 1;
        }
        return -1;
    }
}
