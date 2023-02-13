fn main() {
    assert_eq!(Solution::validate_stack_sequences(vec![1,2,3,4,5], vec![4,5,3,2,1]), true);
    assert_eq!(Solution::validate_stack_sequences(vec![1,2,3,4,5], vec![4,3,5,1,2]), false);
}

struct Solution;
impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut t1 = vec![];
        for p in pushed {
            t1.push(p);
        }
        // for p in popped {
        //     if t1.pop()
        // }
        return false;
    }
}
