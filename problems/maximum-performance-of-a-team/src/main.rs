fn main() {
    assert_eq!(
        Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 2),
        60
    );
    assert_eq!(
        Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 3),
        68
    );
    assert_eq!(
        Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 4),
        72
    );
}

struct Solution;
impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let mut engineers: Vec<(i32, i32)> = speed
            .iter()
            .zip(efficiency.iter())
            .map(|(s, e)| (*s, *e))
            .collect();
        engineers.sort_by(|a, b| b.1.cmp(&a.1));
        let mut max_performance = 0;
        let mut total_speed = 0;
        let mut min_heap = std::collections::BinaryHeap::new();
        for (s, e) in engineers {
            if min_heap.len() as i32 > k - 1 {
                total_speed -= min_heap.pop().unwrap();
            }
            min_heap.push(s);
            total_speed += s;
            max_performance = std::cmp::max(max_performance, total_speed * e);
        }
        return max_performance % 1_000_000_007;
    }
}
