fn main() {
    assert_eq!(
        Solution::xor_after_queries(
            vec![1, 1, 1],
            vec![[0, 2, 1, 4]].iter().map(|v| v.to_vec()).collect()
        ),
        4
    );
    assert_eq!(
        Solution::xor_after_queries(
            vec![2, 3, 1, 5, 4],
            vec![[1, 4, 2, 3], [0, 2, 1, 2]]
                .iter()
                .map(|v| v.to_vec())
                .collect()
        ),
        31
    );
}

struct Solution;
impl Solution {
    pub fn xor_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        for query in queries {
            let (a, b, c, d) = (
                query[0] as usize,
                query[1] as usize,
                query[2] as usize,
                query[3] as usize,
            );
            for i in a..=b {
                nums[i] += c as i64;
            }
            for i in b + 1..=d {
                if i >= nums.len() {
                    break;
                }
                nums[i] += c as i64;
            }
        }
        nums.into_iter().fold(0, |acc, x| acc ^ x as i32)
    }
}
