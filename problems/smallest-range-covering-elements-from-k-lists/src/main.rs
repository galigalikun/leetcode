fn main() {
    assert_eq!(
        Solution::smallest_range(vec![
            vec![4, 10, 15, 24, 26],
            vec![0, 9, 12, 20],
            vec![5, 18, 22, 30]
        ]),
        vec![20, 24]
    );
    assert_eq!(
        Solution::smallest_range(vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]]),
        vec![1, 1]
    );
}



struct Solution {}
use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        if nums.is_empty() || nums.iter().any(Vec::is_empty) {
            return vec![];
        }

        // Min-heap by current value: (value, list_index, element_index)
        let mut min_heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
        let mut current_max = i32::MIN;

        for (list_index, list) in nums.iter().enumerate() {
            let value = list[0];
            min_heap.push(Reverse((value, list_index, 0)));
            if value > current_max {
                current_max = value;
            }
        }

        let mut best_left = 0;
        let mut best_right = i32::MAX;

        // Keep one representative from every list in the heap.
        while min_heap.len() == nums.len() {
            let Reverse((min_value, list_index, element_index)) = min_heap.pop().unwrap();

            // Compare by width first, then by smaller start.
            if (current_max - min_value) < (best_right - best_left)
                || ((current_max - min_value) == (best_right - best_left) && min_value < best_left)
            {
                best_left = min_value;
                best_right = current_max;
            }

            let next_index = element_index + 1;
            if next_index >= nums[list_index].len() {
                break;
            }

            let next_value = nums[list_index][next_index];
            min_heap.push(Reverse((next_value, list_index, next_index)));
            if next_value > current_max {
                current_max = next_value;
            }
        }

        vec![best_left, best_right]
    }
}
