fn main() {
    assert_eq!(Solution::sequential_digits(100, 300), vec![123, 234]);
    assert_eq!(
        Solution::sequential_digits(1000, 13000),
        vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]
    );
}

struct Solution;
impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut result = vec![];
        for i in 1..10 {
            let mut num = i;
            for j in i + 1..10 {
                num = num * 10 + j;
                if num >= low && num <= high {
                    result.push(num);
                }
            }
        }
        result.sort();
        return result;
    }
}
