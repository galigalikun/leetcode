fn main() {
    assert_eq!(
        Solution::eaten_apples(vec![1, 2, 3, 5, 2], vec![3, 2, 1, 4, 2]),
        7
    );
    assert_eq!(
        Solution::eaten_apples(vec![3, 0, 0, 0, 0, 2], vec![3, 0, 0, 0, 0, 2]),
        5
    );
}

struct Solution;
impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let mut eaten = 0;
        let mut heap = std::collections::BinaryHeap::new();
        for i in 0..apples.len() {
            if apples[i] > 0 {
                heap.push((i as i32 + days[i], apples[i]));
            }
            while let Some((day, count)) = heap.peek() {
                if *day <= i as i32 {
                    heap.pop();
                } else {
                    break;
                }
            }
            if let Some((_, count)) = heap.peek() {
                eaten += 1;
                if *count > 1 {
                    heap.push((i as i32 + days[i], count - 1));
                }
            }
        }
        while let Some((day, _)) = heap.peek() {
            if *day <= apples.len() as i32 {
                heap.pop();
            } else {
                break;
            }
        }
        while let Some((_, _)) = heap.peek() {
            eaten += 1;
            heap.pop();
        }
        eaten
    }
}
