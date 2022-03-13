fn main() {
    assert_eq!(
        Solution::island_perimeter(vec![
            vec![0, 1, 0, 0],
            vec![1, 1, 1, 0],
            vec![0, 1, 0, 0],
            vec![1, 1, 0, 0]
        ]),
        16
    );
    assert_eq!(Solution::island_perimeter(vec![vec![1]]), 4);
    assert_eq!(Solution::island_perimeter(vec![vec![1, 0]]), 4);
}

struct Solution {}
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                if grid[x][y] == 1 {
                    result += 4;
                    if x > 0 {
                        if grid[x - 1][y] == 1 {
                            result -= 1;
                        }
                    }
                    if x < grid.len() - 1 {
                        if grid[x + 1][y] == 1 {
                            result -= 1;
                        }
                    }
                    if y > 0 {
                        if grid[x][y - 1] == 1 {
                            result -= 1;
                        }
                    }
                    if y < grid[x].len() - 1 {
                        if grid[x][y + 1] == 1 {
                            result -= 1;
                        }
                    }
                }
            }
        }
        return result;
    }
}
