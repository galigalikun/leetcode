fn main() {
    assert!((Solution::mincost_to_hire_workers(vec![10, 20, 5], vec![70, 50, 30], 2) - 105.0).abs() < 1e-5);
    assert!((Solution::mincost_to_hire_workers(vec![3, 1, 10, 10, 1], vec![4, 8, 2, 2, 7], 3) - 30.6666666667).abs() < 1e-5);
}

struct Solution;
impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        use std::cmp::Ordering;
        use std::collections::BinaryHeap;

        let mut workers: Vec<(f64, i32)> = quality
            .into_iter()
            .zip(wage)
            .map(|(q, w)| (w as f64 / q as f64, q))
            .collect();

        workers.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));

        let k = k as usize;
        let mut max_heap = BinaryHeap::new();
        let mut quality_sum: i64 = 0;
        let mut answer = f64::INFINITY;

        for (ratio, q) in workers {
            max_heap.push(q);
            quality_sum += q as i64;

            if max_heap.len() > k {
                if let Some(removed) = max_heap.pop() {
                    quality_sum -= removed as i64;
                }
            }

            if max_heap.len() == k {
                let cost = ratio * quality_sum as f64;
                if cost < answer {
                    answer = cost;
                }
            }
        }

        answer
    }
}
