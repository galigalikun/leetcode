fn main() {
    assert_eq!(
        Solution::minimum_boxes(vec![1, 3, 2], vec![4, 3, 1, 5, 2]),
        2
    );
    assert_eq!(Solution::minimum_boxes(vec![5, 5, 5], vec![2, 4, 2, 7]), 4);
}

struct Solution;
impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
        let mut apple = apple;
        let mut capacity = capacity;
        apple.sort_unstable_by(|a, b| b.cmp(a));
        capacity.sort_unstable();
        let mut i = 0;
        let mut j = 0;
        let mut res = 0;
        while i < apple.len() && j < capacity.len() {
            if capacity[j] >= apple[i] {
                res += 1;
                i += 1;
            }
            j += 1;
        }
        if i == apple.len() { res } else { -1 }
    }
}
