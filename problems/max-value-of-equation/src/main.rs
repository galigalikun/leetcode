fn main() {
    assert_eq!(
        Solution::find_max_value_of_equation(
            vec![vec![1, 3], vec![2, 0], vec![5, 10], vec![6, -10]],
            1
        ),
        4
    );
    assert_eq!(
        Solution::find_max_value_of_equation(vec![vec![0, 0], vec![3, 0], vec![9, 2]], 3),
        3
    );
    assert_eq!(
        Solution::find_max_value_of_equation(vec![vec![-19, 9], vec![-15, -19], vec![-5, -8]], 10),
        -6
    );
}

struct Solution;
impl Solution {
    pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut max = 0;
        let mut deque = std::collections::VecDeque::new();
        for point in points {
            let x = point[0];
            let y = point[1];
            let (a, _) = *deque.front().unwrap_or(&(-1, -1));
            while !deque.is_empty() && x - a > k {
                deque.pop_front();
            }
            if !deque.is_empty() {
                max = std::cmp::max(max, x + y + deque.front().unwrap().1);
            }
            while !deque.is_empty() && deque.back().unwrap().1 <= y - x {
                deque.pop_back();
            }
            deque.push_back((x, y - x));
        }
        return max;
    }
}
