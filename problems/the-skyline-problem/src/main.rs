fn main() {
    assert_eq!(
        Solution::get_skyline(vec![
            vec![2, 9, 10],
            vec![3, 7, 15],
            vec![5, 12, 12],
            vec![15, 20, 10],
            vec![19, 24, 8]
        ]),
        vec![
            [2, 10],
            [3, 15],
            [7, 12],
            [12, 0],
            [15, 10],
            [20, 8],
            [24, 0]
        ]
    );

    assert_eq!(
        Solution::get_skyline(vec![vec![0, 2, 3], vec![2, 5, 3]]),
        vec![[0, 3], [5, 0]]
    );
}

struct Solution {}
// https://www.geeksforgeeks.org/the-skyline-problem-set-2/
impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut wall = vec![];
        let n = buildings.len();
        for i in 0..n {
            let left = buildings[i][0];
            let height = buildings[i][2];
            let right = buildings[i][1];
            wall.push(vec![left, -height]);
            wall.push(vec![right, height]);
        }
        wall.sort();

        let mut top = 0;
        let mut left_wall_height = vec![0];
        let mut result = vec![];
        for w in wall {
            if w[1] < 0 {
                left_wall_height.push(-w[1]);
            } else {
                if let Some(p) = left_wall_height.iter().position(|&h| h == w[1]) {
                    left_wall_height.remove(p);
                }
            }
            left_wall_height.sort();

            if let Some(&h) = left_wall_height.last() {
                if h != top {
                    top = h;
                    result.push(vec![w[0], top]);
                }
            }
        }
        return result;
    }
}
