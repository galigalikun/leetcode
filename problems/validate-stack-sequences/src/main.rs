fn main() {
    assert!(Solution::validate_stack_sequences(
        vec![1, 2, 3, 4, 5],
        vec![4, 5, 3, 2, 1]
    ));
    assert!(!Solution::validate_stack_sequences(
        vec![1, 2, 3, 4, 5],
        vec![4, 3, 5, 1, 2]
    ));
}

struct Solution;
impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = Vec::with_capacity(pushed.len());
        let mut pop_index = 0;

        for value in pushed {
            stack.push(value);

            while let Some(&top) = stack.last() {
                if pop_index >= popped.len() || top != popped[pop_index] {
                    break;
                }
                stack.pop();
                pop_index += 1;
            }
        }

        pop_index == popped.len()
    }
}
