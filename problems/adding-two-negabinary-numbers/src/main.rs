fn main() {
    assert_eq!(
        Solution::add_negabinary(vec![1, 1, 1, 1, 1], vec![1, 0, 1]),
        vec![1, 0, 0, 0, 0]
    );
    assert_eq!(Solution::add_negabinary(vec![0], vec![0]), vec![0]);
    assert_eq!(Solution::add_negabinary(vec![0], vec![1]), vec![1]);
    assert_eq!(Solution::add_negabinary(vec![1], vec![1]), vec![1, 1, 0]);
}

struct Solution;
impl Solution {
    pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut arr1 = arr1;
        let mut arr2 = arr2;
        let mut carry = 0;
        let mut result = vec![];
        while !arr1.is_empty() || !arr2.is_empty() || carry != 0 {
            let mut sum = carry;
            if !arr1.is_empty() {
                sum += arr1.pop().unwrap();
            }
            if !arr2.is_empty() {
                sum += arr2.pop().unwrap();
            }
            result.push(sum % 2);
            carry = -(sum >> 1);
        }
        while result.len() > 1 && result.last().unwrap() == &0 {
            result.pop();
        }
        result.reverse();
        return result;
    }
}
