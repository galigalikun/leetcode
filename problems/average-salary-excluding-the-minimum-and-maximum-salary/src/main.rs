fn main() {
    assert_eq!(Solution::average(vec![4000, 3000, 1000, 2000]), 2500.00000);
    assert_eq!(Solution::average(vec![1000, 2000, 3000]), 2000.00000);
}

struct Solution;
impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut sum = 0;
        for s in salary.iter() {
            if *s < min {
                min = *s;
            }
            if *s > max {
                max = *s;
            }
            sum += *s;
        }
        sum -= min;
        sum -= max;
        return sum as f64 / (salary.len() as f64 - 2.0);
    }
}
