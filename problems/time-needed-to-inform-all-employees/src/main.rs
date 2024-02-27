fn main() {
    assert_eq!(Solution::num_of_minutes(1, 0, vec![-1], vec![0]), 0);
    assert_eq!(
        Solution::num_of_minutes(6, 2, vec![2, 2, -1, 2, 2, 2], vec![0, 0, 1, 0, 0, 0]),
        1
    );
}

struct Solution;
impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..n {
            if inform_time[i as usize] == 0 {
                let mut j = i;
                let mut time = 0;
                while j != head_id {
                    time += inform_time[j as usize];
                    j = manager[j as usize];
                }
                result = result.max(time);
            }
        }
        return result;
    }
}
