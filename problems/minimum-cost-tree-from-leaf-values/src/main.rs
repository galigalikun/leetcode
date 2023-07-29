fn main() {
    assert_eq!(Solution::mct_from_leaf_values(vec![6, 2, 4]), 32);
    assert_eq!(Solution::mct_from_leaf_values(vec![4, 11]), 44);
}

struct Solution;
impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let mut stack = vec![i32::MAX];
        let mut res = 0;
        for i in 0..arr.len() {
            while stack.last().unwrap() <= &arr[i] {
                let mid = stack.pop().unwrap();
                res += mid * std::cmp::min(stack.last().unwrap(), &arr[i]);
            }
            stack.push(arr[i]);
        }
        while stack.len() > 2 {
            let mid = stack.pop().unwrap();
            res += mid * stack.last().unwrap();
        }
        return res;
    }
}
