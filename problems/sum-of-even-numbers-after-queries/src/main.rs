fn main() {
    assert_eq!(Solution::sum_even_after_queries(vec![1,2,3,4], vec![vec![1,0],vec![-3,1],vec![-4,0],vec![2,3]]), vec![8,6,2,4]);
    assert_eq!(Solution::sum_even_after_queries(vec![1], vec![vec![4,0]]), vec![0]);
}

struct Solution;
impl Solution {
    pub fn sum_even_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        return vec![];
    }
}
