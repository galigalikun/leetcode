fn main() {
    assert_eq!(
        Solution::contain_virus(vec![
            vec![0, 1, 0, 0, 0, 0, 0, 1],
            vec![0, 1, 0, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0]
        ]),
        10
    );

    assert_eq!(
        Solution::contain_virus(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
        4
    );
    assert_eq!(
        Solution::contain_virus(vec![
            vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 0, 0, 0, 0, 0, 0]
        ]),
        13
    );
}

use std::collections::HashSet;

const QUARANTINED: i32 = 2;
const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

/// A connected block of infected cells, discovered during one night's scan.
struct Region {
    cells: Vec<(usize, usize)>,
    /// Distinct uninfected cells this region threatens next night.
    threatened: HashSet<(usize, usize)>,
    /// Walls needed to fully seal this region (counts shared boundaries with multiplicity).
    walls: i32,
}

struct Solution {}
impl Solution {
    pub fn contain_virus(is_infected: Vec<Vec<i32>>) -> i32 {
        let mut grid = is_infected;
        let mut total_walls = 0;

        loop {
            let regions = Self::scan_regions(&grid);

            // The most threatening active region; none means the spread is over.
            let target = regions
                .iter()
                .filter(|region| !region.threatened.is_empty())
                .max_by_key(|region| region.threatened.len());

            let target = match target {
                Some(region) => region,
                None => break,
            };

            total_walls += target.walls;

            // Seal the chosen region, then let every other region spread for the night.
            for region in &regions {
                if std::ptr::eq(region, target) {
                    for &(row, col) in &region.cells {
                        grid[row][col] = QUARANTINED;
                    }
                } else {
                    for &(row, col) in &region.threatened {
                        grid[row][col] = 1;
                    }
                }
            }
        }

        total_walls
    }

    /// Discover every connected block of active virus (value `1`) and its frontier.
    fn scan_regions(grid: &[Vec<i32>]) -> Vec<Region> {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut visited = vec![vec![false; cols]; rows];
        let mut regions = Vec::new();

        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == 1 && !visited[row][col] {
                    regions.push(Self::explore_region(grid, &mut visited, row, col));
                }
            }
        }

        regions
    }

    /// Flood-fill one region starting at `(start_row, start_col)`.
    fn explore_region(
        grid: &[Vec<i32>],
        visited: &mut [Vec<bool>],
        start_row: usize,
        start_col: usize,
    ) -> Region {
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;
        let mut cells = Vec::new();
        let mut threatened = HashSet::new();
        let mut walls = 0;

        let mut stack = vec![(start_row, start_col)];
        visited[start_row][start_col] = true;

        while let Some((row, col)) = stack.pop() {
            cells.push((row, col));

            for (delta_row, delta_col) in DIRECTIONS {
                let next_row = row as i32 + delta_row;
                let next_col = col as i32 + delta_col;
                if next_row < 0 || next_row >= rows || next_col < 0 || next_col >= cols {
                    continue;
                }
                let (next_row, next_col) = (next_row as usize, next_col as usize);

                match grid[next_row][next_col] {
                    0 => {
                        walls += 1;
                        threatened.insert((next_row, next_col));
                    }
                    1 if !visited[next_row][next_col] => {
                        visited[next_row][next_col] = true;
                        stack.push((next_row, next_col));
                    }
                    _ => {}
                }
            }
        }

        Region {
            cells,
            threatened,
            walls,
        }
    }
}
