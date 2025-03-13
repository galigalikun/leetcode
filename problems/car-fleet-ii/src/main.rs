fn main() {
    assert_eq!(
        Solution::get_collision_times(vec![vec![1, 2], vec![2, 1], vec![4, 3], vec![7, 2]]),
        vec![1.00000, -1.00000, 3.00000, -1.00000]
    );
    assert_eq!(
        Solution::get_collision_times(vec![vec![3, 4], vec![5, 4], vec![6, 3], vec![9, 1]]),
        vec![2.00000, 1.00000, 1.50000, -1.00000]
    );
}

struct Solution;
impl Solution {
    pub fn get_collision_times(cars: Vec<Vec<i32>>) -> Vec<f64> {
        let n = cars.len();
        let mut stack = vec![];
        let mut res = vec![-1.0; n];
        for i in (0..n).rev() {
            let (pos, speed) = (cars[i][0] as f64, cars[i][1] as f64);
            while !stack.is_empty() {
                let j: usize = stack[stack.len() - 1];
                let (pos2, speed2) = (cars[j][0] as f64, cars[j][1] as f64);
                if speed <= speed2 || (res[j] > 0.0 && (pos2 - pos) / (speed - speed2) >= res[j]) {
                    stack.pop();
                } else {
                    break;
                }
            }
            if !stack.is_empty() {
                let j = stack[stack.len() - 1];
                res[i] = (cars[j][0] as f64 - pos) / (speed - cars[j][1] as f64);
            }
            stack.push(i);
        }
        res
    }
}
