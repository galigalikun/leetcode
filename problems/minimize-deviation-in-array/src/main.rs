fn main() {
    assert_eq!(Solution::minimum_deviation(vec![1, 2, 3, 4]), 1);
    assert_eq!(Solution::minimum_deviation(vec![4, 1, 5, 20, 3]), 3);
    assert_eq!(Solution::minimum_deviation(vec![2, 10, 8]), 3);
}

struct Solution;
impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        let mut max = 0;
        for mut num in nums {
            if num % 2 == 1 {
                num *= 2;
            }
            heap.push(num);
            max = max.max(num);
        }
        let mut min = max;
        let mut result = max - heap.peek().unwrap();
        while let Some(mut num) = heap.pop() {
            if num % 2 == 0 {
                num /= 2;
                heap.push(num);
                min = min.min(num);
                result = result.min(max - heap.peek().unwrap());
            } else {
                break;
            }
        }
        result
    }
}
