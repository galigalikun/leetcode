fn main() {
    assert_eq!(
        Solution::xor_queries(
            vec![1, 3, 4, 8],
            vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]]
        ),
        vec![2, 7, 14, 8]
    );
    assert_eq!(
        Solution::xor_queries(
            vec![4, 8, 2, 10],
            vec![vec![2, 3], vec![1, 3], vec![0, 0], vec![0, 3]]
        ),
        vec![8, 0, 4, 4]
    );
}

struct Solution;
impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        for query in queries {
            let mut xor = 0;
            for i in query[0]..=query[1] {
                xor ^= arr[i as usize];
            }
            result.push(xor);
        }
        return result;
    }
}
