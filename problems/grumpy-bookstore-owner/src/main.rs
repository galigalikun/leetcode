fn main() {
    assert_eq!(Solution::max_satisfied(vec![1, 0, 1, 2, 1, 1, 7, 5], vec![0, 1, 0, 1, 0, 1, 0, 1], 3), 16);
    assert_eq!(Solution::max_satisfied(vec![1], vec![0], 1), 1);
    assert_eq!(Solution::max_satisfied(vec![4, 10, 10], vec![1, 1, 0], 2), 24);
    assert_eq!(Solution::max_satisfied(vec![1, 3], vec![0, 0], 1), 4);
}

struct Solution;
impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let window = minutes as usize;
        let n = customers.len();
        let mut base = 0;
        for i in 0..n {
            if grumpy[i] == 0 {
                base += customers[i];
            }
        }

        let mut window_gain = 0;
        for i in 0..n.min(window) {
            if grumpy[i] == 1 {
                window_gain += customers[i];
            }
        }

        let mut best_gain = window_gain;
        for i in window..n {
            if grumpy[i - window] == 1 {
                window_gain -= customers[i - window];
            }
            if grumpy[i] == 1 {
                window_gain += customers[i];
            }
            best_gain = best_gain.max(window_gain);
        }

        base + best_gain
    }
}
