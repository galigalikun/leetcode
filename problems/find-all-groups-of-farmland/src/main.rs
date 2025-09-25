fn main() {
    assert_eq!(
        Solution::find_farmland(vec![vec![1, 0, 0], vec![0, 1, 1], vec![0, 1, 1]]),
        vec![vec![0, 0, 0, 0], vec![1, 1, 2, 2]]
    );
    assert_eq!(
        Solution::find_farmland(vec![vec![1, 1], vec![1, 1]]),
        vec![vec![0, 0, 1, 1]]
    );
}

struct Solution;
impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut visited = vec![vec![false; land[0].len()]; land.len()];
        let mut result = Vec::new();

        for i in 0..land.len() {
            for j in 0..land[0].len() {
                if land[i][j] == 1 && !visited[i][j] {
                    let x1 = i;
                    let y1 = j;
                    let mut x2 = i;
                    let mut y2 = j;

                    while x2 + 1 < land.len() && land[x2 + 1][j] == 1 {
                        x2 += 1;
                    }
                    while y2 + 1 < land[0].len() && land[i][y2 + 1] == 1 {
                        y2 += 1;
                    }

                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            visited[x][y] = true;
                        }
                    }

                    result.push(vec![x1 as i32, y1 as i32, x2 as i32, y2 as i32]);
                }
            }
        }

        result
    }
}
