fn main() {
    assert_eq!(Solution::get_descent_periods(vec![3, 2, 1, 4]), 7);
    assert_eq!(Solution::get_descent_periods(vec![8, 6, 7, 7]), 4);
    assert_eq!(Solution::get_descent_periods(vec![1]), 1);
}

struct Solution;
impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        return prices
            .iter()
            .fold((0, 0, 0), |(mut total, mut count, mut prev), &price| {
                if prev == 0 || price == prev - 1 {
                    count += 1;
                } else {
                    count = 1;
                }
                total += count;
                prev = price;
                (total, count, prev)
            })
            .0;
    }
}
