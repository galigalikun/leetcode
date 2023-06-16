fn main() {
    assert_eq!(Solution::max_satisfied(vec![1,0,1,2,1,1,7,5], vec![0,1,0,1,0,1,0,1], 3), 16);
    assert_eq!(Solution::max_satisfied(vec![1], vec![0], 1), 1);
}

struct Solution;
impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let mut max = 0;
        let mut sum = 0;
        let mut sum_grumpy = 0;
        let mut sum_grumpy_max = 0;
        let mut sum_grumpy_max_index = 0;
        let mut sum_grumpy_max_index_end = 0;
        let mut sum_grumpy_max_index_end_max = 0;
        for i in 0..customers.len() {
            sum += customers[i];
            sum_grumpy += customers[i] * grumpy[i];
            if i >= minutes as usize {
                sum_grumpy -= customers[i - minutes as usize] * grumpy[i - minutes as usize];
            }
            if sum_grumpy > sum_grumpy_max {
                sum_grumpy_max = sum_grumpy;
                sum_grumpy_max_index = i;
            }
            if sum_grumpy_max_index_end_max < sum_grumpy_max + sum - sum_grumpy {
                sum_grumpy_max_index_end_max = sum_grumpy_max + sum - sum_grumpy;
                sum_grumpy_max_index_end = i;
            }
        }
        if sum_grumpy_max_index_end_max > sum_grumpy_max {
            return sum_grumpy_max_index_end_max;
        }
        return 0;
    }
}
