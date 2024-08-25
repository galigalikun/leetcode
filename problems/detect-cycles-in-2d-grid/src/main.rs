fn main() {
    assert_eq!(
        Solution::contains_cycle(vec![
            vec!['a', 'a', 'a', 'a'],
            vec!['a', 'b', 'b', 'a'],
            vec!['a', 'b', 'b', 'a'],
            vec!['a', 'a', 'a', 'a']
        ]),
        true
    );
    assert_eq!(
        Solution::contains_cycle(vec![
            vec!['c', 'c', 'c', 'a'],
            vec!['c', 'd', 'c', 'c'],
            vec!['c', 'c', 'e', 'c'],
            vec!['f', 'c', 'c', 'c']
        ]),
        true
    );
    assert_eq!(
        Solution::contains_cycle(vec![
            vec!['a', 'b', 'b'],
            vec!['b', 'z', 'b'],
            vec!['b', 'b', 'a']
        ]),
        false
    );
}

struct Solution;
impl Solution {
    fn dfs(
        i: usize,
        j: usize,
        visited: &mut Vec<Vec<bool>>,
        parent: &mut Vec<Vec<i32>>,
        grid: &Vec<Vec<char>>,
        p: i32,
    ) -> bool {
        visited[i][j] = true;
        parent[i][j] = p;
        let mut cycle = false;
        let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in directions {
            let x = i as i32 + dx;
            let y = j as i32 + dy;
            if visited[x as usize][y as usize] {
                if parent[i][j] != parent[x as usize][y as usize] {
                    cycle = true;
                    break;
                }
            } else if grid[i][j] == grid[x as usize][y as usize] {
                cycle = Solution::dfs(x as usize, y as usize, visited, parent, grid, parent[i][j]);
                if cycle {
                    break;
                }
            }
        }
        cycle
    }
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut parent = vec![vec![-1; grid[0].len()]; grid.len()];
        let mut cycle = false;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if !visited[i][j] {
                    if Solution::dfs(i, j, &mut visited, &mut parent, &grid, -1) {
                        cycle = true;
                        break;
                    }
                }
            }
        }
        cycle
    }
}
