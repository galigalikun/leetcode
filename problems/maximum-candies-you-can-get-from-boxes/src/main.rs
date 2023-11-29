fn main() {
    assert_eq!(
        Solution::max_candies(
            vec![1, 0, 1, 0],
            vec![7, 5, 4, 100],
            vec![vec![], vec![], vec![1], vec![]],
            vec![vec![1, 2], vec![3], vec![], vec![]],
            vec![0]
        ),
        16
    );
    assert_eq!(
        Solution::max_candies(
            vec![1, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1, 1],
            vec![vec![], vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![]],
            vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]],
            vec![0]
        ),
        6
    );
}

struct Solution;
impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut boxes = initial_boxes;
        let keys = keys;
        let contained_boxes = contained_boxes;
        let candies = candies;
        let mut status = status;
        let mut res = 0;
        let mut flag = true;
        while flag {
            flag = false;
            for i in 0..boxes.len() {
                if status[boxes[i] as usize] == 1 {
                    res += candies[boxes[i] as usize];
                    status[boxes[i] as usize] = 2;
                    flag = true;
                    for j in 0..contained_boxes[boxes[i] as usize].len() {
                        boxes.push(contained_boxes[boxes[i] as usize][j]);
                    }
                    for j in 0..keys[boxes[i] as usize].len() {
                        status[keys[boxes[i] as usize][j] as usize] = 1;
                    }
                }
            }
        }
        return res;
    }
}
