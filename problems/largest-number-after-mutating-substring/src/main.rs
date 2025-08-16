fn main() {
    assert_eq!(
        Solution::maximum_number("132".to_string(), vec![9, 8, 5, 0, 3, 6, 4, 2, 6, 8]),
        "832".to_string()
    );
    assert_eq!(
        Solution::maximum_number("021".to_string(), vec![9, 4, 3, 5, 7, 2, 1, 9, 0, 6]),
        "934".to_string()
    );
    assert_eq!(
        Solution::maximum_number("5".to_string(), vec![1, 4, 7, 5, 3, 2, 5, 6, 9, 4]),
        "5"
    );
}

struct Solution;
impl Solution {
    pub fn maximum_number(num: String, change: Vec<i32>) -> String {
        return num
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let digit = c.to_digit(10).unwrap() as i32;
                if digit < change.len() as i32 && change[digit as usize] > digit {
                    change[digit as usize].to_string()
                } else {
                    c.to_string()
                }
            })
            .collect();
    }
}
