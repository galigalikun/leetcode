fn main() {
    assert_eq!(
        Solution::nearest_exit(
            vec![
                vec!['+', '+', '.', '+'],
                vec!['.', '.', '.', '+'],
                vec!['+', '+', '+', '.'],
            ],
            vec![1, 2]
        ),
        1
    );
    assert_eq!(
        Solution::nearest_exit(
            vec![
                vec!['+', '+', '+'],
                vec!['.', '.', '.'],
                vec!['+', '+', '+'],
            ],
            vec![1, 0]
        ),
        2
    );
    assert_eq!(
        Solution::nearest_exit(vec![vec!['.', '+'],], vec![0, 0]),
        -1
    );
}

struct Solution;
impl Solution {
    fn bfs(maze: &Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let rows = maze.len();
        let cols = maze[0].len();
        let mut queue = std::collections::VecDeque::new();
        let mut visited = vec![vec![false; cols]; rows];

        // Directions for moving up, down, left, right
        let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        // Start BFS from the entrance
        queue.push_back((entrance[0] as usize, entrance[1] as usize, 0));
        visited[entrance[0] as usize][entrance[1] as usize] = true;

        while let Some((x, y, steps)) = queue.pop_front() {
            // Check if we are at an exit
            if (x == 0 || x == rows - 1 || y == 0 || y == cols - 1)
                && !(x == entrance[0] as usize && y == entrance[1] as usize)
            {
                return steps;
            }

            // Explore all possible directions
            for &(dx, dy) in &directions {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;

                if new_x >= 0
                    && new_x < rows as isize
                    && new_y >= 0
                    && new_y < cols as isize
                    && maze[new_x as usize][new_y as usize] == '.'
                    && !visited[new_x as usize][new_y as usize]
                {
                    visited[new_x as usize][new_y as usize] = true;
                    queue.push_back((new_x as usize, new_y as usize, steps + 1));
                }
            }
        }

        // If no exit found
        -1
    }
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        return Self::bfs(&maze, entrance);
    }
}
