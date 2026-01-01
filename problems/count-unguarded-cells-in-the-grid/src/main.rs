fn main() {
    assert_eq!(
        Solution::count_unguarded(
            4,
            6,
            vec![vec![0, 0], vec![1, 1], vec![2, 3]],
            vec![[0, 1], [2, 2], [1, 4]]
                .iter()
                .map(|v| v.to_vec())
                .collect(),
        ),
        7
    );
    assert_eq!(
        Solution::count_unguarded(
            3,
            3,
            vec![vec![1, 1]],
            vec![[0, 1], [1, 0], [2, 1], [1, 2]]
                .iter()
                .map(|v| v.to_vec())
                .collect(),
        ),
        4
    );
}

struct Solution;
impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut grid = vec![vec![0u8; n]; m];
        for wall in walls {
            grid[wall[0] as usize][wall[1] as usize] = 2;
        }
        for guard in &guards {
            grid[guard[0] as usize][guard[1] as usize] = 1;
        }
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for guard in guards {
            let (gx, gy) = (guard[0] as usize, guard[1] as usize);
            for &(dx, dy) in &directions {
                let (mut x, mut y) = (gx as i32, gy as i32);
                loop {
                    x += dx;
                    y += dy;
                    if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 {
                        break;
                    }
                    match grid[x as usize][y as usize] {
                        0 => grid[x as usize][y as usize] = 3,
                        1 | 2 => break,
                        _ => {}
                    }
                }
            }
        }
        let mut result = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    result += 1;
                }
            }
        }
        result
    }
}
