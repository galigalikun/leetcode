fn main() {
    assert_eq!(Solution::find_kth_smallest(vec![3, 6, 9], 3), 9);
    assert_eq!(Solution::find_kth_smallest(vec![5, 2], 7), 12);
}

struct Solution;
impl Solution {
    pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        heap.push(0i64);
        for &coin in &coins {
            let coin = coin as i64;
            let mut new_heap = BinaryHeap::new();
            while let Some(sum) = heap.pop() {
                let new_sum = sum + coin;
                new_heap.push(new_sum);
                new_heap.push(sum);
            }
            heap = new_heap;
        }
        let mut result = 0i64;
        for _ in 0..k {
            if let Some(sum) = heap.pop() {
                result = sum;
            }
        }
        result
    }
}
