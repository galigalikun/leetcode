fn main() {
    assert_eq!(Solution::min_stone_sum(vec![5, 4, 9], 2), 12);
    assert_eq!(Solution::min_stone_sum(vec![4, 3, 6, 7], 3), 12);
}

struct Solution;
impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        for &pile in &piles {
            heap.push(pile);
        }
        let mut k = k;
        while k > 0 {
            if let Some(top) = heap.pop() {
                let reduced = top - top / 2;
                heap.push(reduced);
            }
            k -= 1;
        }
        heap.into_iter().sum()
    }
}
