fn main() {
    assert_eq!(
        Solution::is_printable(vec![
            vec![1, 1, 1, 1],
            vec![1, 2, 2, 1],
            vec![1, 2, 2, 1],
            vec![1, 1, 1, 1]
        ]),
        true
    );
    assert_eq!(
        Solution::is_printable(vec![
            vec![1, 1, 1, 1],
            vec![1, 1, 3, 3],
            vec![1, 1, 3, 4],
            vec![5, 5, 1, 4]
        ]),
        true
    );
    assert_eq!(
        Solution::is_printable(vec![vec![1, 2, 1], vec![2, 1, 2], vec![1, 2, 1]]),
        false
    );
}

struct Solution;
impl Solution {
    fn dfs(
        color: usize,
        colors: &Vec<Vec<(i32, i32)>>,
        visited: &mut Vec<bool>,
        stack: &mut Vec<usize>,
        min: &Vec<i32>,
        max: &Vec<i32>,
    ) -> bool {
        visited[color] = true;
        stack.push(color);
        for (i, j) in &colors[color] {
            if min[color] <= *i && *i <= max[color] && min[color] <= *j && *j <= max[color] {
                continue;
            }
            for &next_color in stack.iter() {
                if min[next_color] <= *i
                    && *i <= max[next_color]
                    && min[next_color] <= *j
                    && *j <= max[next_color]
                {
                    return false;
                }
            }
        }
        for &next_color in &colors[color] {
            if visited[next_color.0 as usize] {
                continue;
            }
            if !Self::dfs(next_color.0 as usize, colors, visited, stack, min, max) {
                return false;
            }
        }
        stack.pop();
        true
    }
    pub fn is_printable(target_grid: Vec<Vec<i32>>) -> bool {
        let mut min = vec![100; 61];
        let mut max = vec![0; 61];
        let mut colors = vec![vec![]; 61];
        let n = target_grid.len();
        let m = target_grid[0].len();
        for i in 0..n {
            for j in 0..m {
                let color = target_grid[i][j] as usize;
                min[color] = min[color].min(i as i32).min(j as i32);
                max[color] = max[color].max(i as i32).max(j as i32);
            }
        }
        for i in 0..n {
            for j in 0..m {
                let color = target_grid[i][j] as usize;
                colors[color].push((i as i32, j as i32));
            }
        }
        let mut visited = vec![false; 61];
        let mut stack = vec![];
        for color in 1..61 {
            if colors[color].is_empty() {
                continue;
            }
            if visited[color] {
                continue;
            }
            if !Self::dfs(color, &colors, &mut visited, &mut stack, &min, &max) {
                return false;
            }
        }
        true
    }
}
