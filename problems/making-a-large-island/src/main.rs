use std::collections::HashSet;

fn main() {
    assert_eq!(Solution::largest_island(vec![vec![1,0],vec![0,1]]), 3);
    assert_eq!(Solution::largest_island(vec![vec![1,1],vec![1,0]]), 4);
    assert_eq!(Solution::largest_island(vec![vec![1,1],vec![1,1]]), 4);
}

struct Solution{}
impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if n == 0 {
            return 0;
        }

        let mut component_ids = vec![vec![0_i32; n]; n];
        let mut component_areas = vec![0_i32];
        let mut current_id = 1_i32;

        for row in 0..n {
            for col in 0..n {
                if grid[row][col] == 1 && component_ids[row][col] == 0 {
                    component_areas.push(0);
                    let component_index = current_id as usize;
                    let mut stack = vec![(row, col)];
                    component_ids[row][col] = current_id;

                    while let Some((r, c)) = stack.pop() {
                        component_areas[component_index] += 1;

                        if r > 0 && grid[r - 1][c] == 1 && component_ids[r - 1][c] == 0 {
                            component_ids[r - 1][c] = current_id;
                            stack.push((r - 1, c));
                        }
                        if r + 1 < n && grid[r + 1][c] == 1 && component_ids[r + 1][c] == 0 {
                            component_ids[r + 1][c] = current_id;
                            stack.push((r + 1, c));
                        }
                        if c > 0 && grid[r][c - 1] == 1 && component_ids[r][c - 1] == 0 {
                            component_ids[r][c - 1] = current_id;
                            stack.push((r, c - 1));
                        }
                        if c + 1 < n && grid[r][c + 1] == 1 && component_ids[r][c + 1] == 0 {
                            component_ids[r][c + 1] = current_id;
                            stack.push((r, c + 1));
                        }
                    }

                    current_id += 1;
                }
            }
        }

        let mut has_zero = false;
        let mut best = component_areas.iter().copied().max().unwrap_or(0);

        for row in 0..n {
            for col in 0..n {
                if grid[row][col] == 0 {
                    has_zero = true;
                    let mut neighbor_components = HashSet::new();

                    if row > 0 {
                        neighbor_components.insert(component_ids[row - 1][col]);
                    }
                    if row + 1 < n {
                        neighbor_components.insert(component_ids[row + 1][col]);
                    }
                    if col > 0 {
                        neighbor_components.insert(component_ids[row][col - 1]);
                    }
                    if col + 1 < n {
                        neighbor_components.insert(component_ids[row][col + 1]);
                    }

                    let merged_area = 1
                        + neighbor_components
                            .iter()
                            .copied()
                            .filter(|component_id| *component_id > 0)
                            .map(|component_id| component_areas[component_id as usize])
                            .sum::<i32>();

                    best = best.max(merged_area);
                }
            }
        }

        if has_zero { best } else { (n * n) as i32 }
    }
}
