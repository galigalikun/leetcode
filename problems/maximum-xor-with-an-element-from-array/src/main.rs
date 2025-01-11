fn main() {
    assert_eq!(
        Solution::maximize_xor(
            vec![0, 1, 2, 3, 4],
            vec![vec![3, 1], vec![1, 3], vec![5, 6]]
        ),
        vec![3, 3, 7]
    );
    assert_eq!(
        Solution::maximize_xor(
            vec![5, 2, 4, 6, 6, 3],
            vec![vec![12, 4], vec![8, 1], vec![6, 3]]
        ),
        vec![15, -1, 5]
    );
}

struct Solution;
impl Solution {
    pub fn maximize_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![];
        for query in queries {
            let mut max = -1;
            for num in nums.iter() {
                if num >= &query[0] && num <= &query[1] {
                    max = max.max(num ^ query[1]);
                }
            }
            res.push(max);
        }
        res
    }
}
