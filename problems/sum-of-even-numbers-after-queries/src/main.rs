fn main() {
    assert_eq!(
        Solution::sum_even_after_queries(
            vec![1, 2, 3, 4],
            vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]]
        ),
        vec![8, 6, 2, 4]
    );
    assert_eq!(
        Solution::sum_even_after_queries(vec![1], vec![vec![4, 0]]),
        vec![]
    );
}

struct Solution;
impl Solution {
    pub fn sum_even_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sum = 0;
        for num in nums.iter() {
            if num % 2 == 0 {
                sum += num;
            }
        }
        let mut result = vec![];
        for query in queries.iter() {
            let val = query[0];
            let index = query[1] as usize;
            if nums[index] % 2 == 0 {
                sum -= nums[index];
            }
            let new_num = nums[index] + val;
            if new_num % 2 == 0 {
                sum += new_num;
            }
            result.push(sum);
        }
        result
    }
}
