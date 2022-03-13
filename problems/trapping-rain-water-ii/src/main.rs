fn main() {
    assert_eq!(
        Solution::trap_rain_water(vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 1, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1]
        ]),
        4
    );
}

struct Solution {}
// https://medium.com/@rajwar67/solving-trapping-rain-water-ii-2d-elevation-map-problem-2e52aa27cd39
impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let row = height_map.len() as i32;
        if row == 0 {
            return 0;
        }
        let col = height_map[0].len() as i32;
        let mut visited = vec![vec![false; col as usize]; row as usize];
        let mut min_heap = vec![];
        for i in 0..row {
            min_heap.push((height_map[i as usize][0], i, 0));
            min_heap.push((height_map[i as usize][(col - 1) as usize], i, col - 1));
            visited[i as usize][0] = true;
            visited[i as usize][(col - 1) as usize] = true;
        }

        for j in 1..col - 1 {
            min_heap.push((height_map[0][j as usize], 0, j));
            min_heap.push((height_map[(row - 1) as usize][j as usize], row - 1, j));
            visited[0][j as usize] = true;
            visited[(row - 1) as usize][j as usize] = true;
        }

        min_heap.sort_by(|a, b| a.0.cmp(&b.0));

        let mut current = 0;
        let mut result = 0;
        let dirs: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        while !min_heap.is_empty() {
            let cell = min_heap.remove(0);
            current = std::cmp::max(current, cell.0);
            for dir in &dirs {
                let i = cell.1 + dir.0;
                let j = cell.2 + dir.1;
                if i >= 0 && i < row && j >= 0 && j < col && !visited[i as usize][j as usize] {
                    if current > height_map[i as usize][j as usize] {
                        result += current - height_map[i as usize][j as usize];
                    }
                    min_heap.push((height_map[i as usize][j as usize], i, j));
                    min_heap.sort_by(|a, b| a.0.cmp(&b.0));
                    visited[i as usize][j as usize] = true;
                }
            }
        }
        return result;
    }
}
