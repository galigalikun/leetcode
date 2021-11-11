fn main() {
    assert_eq!(Solution::convert_to_base7(100), "202");
    assert_eq!(Solution::convert_to_base7(-7), "-10");
}

struct Solution {}
impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        let mut n = num.abs();
        let mut result = vec![];
        loop {
            let b = n % 7;
            n = n / 7;
            result.push(std::char::from_digit(b as u32, 10).unwrap());
            if n < 7 {
                result.push(std::char::from_digit(n as u32, 10).unwrap());
                break;
            }
        }

        return if num >= 0 {
            format!(
                "{:?}",
                result
                    .iter()
                    .rev()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap()
            )
        } else {
            format!(
                "-{:?}",
                result
                    .iter()
                    .rev()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap()
            )
        };
    }
}
