fn main() {
    assert_eq!(Solution::fizz_buzz(3), vec!["1", "2", "Fizz"]);
    assert_eq!(Solution::fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
    assert_eq!(
        Solution::fizz_buzz(15),
        vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz"
        ]
    );
}

pub struct Solution {}
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        return (1..=n)
            .map(|x| {
                if x % 15 == 0 {
                    "FizzBuzz".to_string()
                } else if x % 3 == 0 {
                    "Fizz".to_string()
                } else if x % 5 == 0 {
                    "Buzz".to_string()
                } else {
                    x.to_string()
                }
            })
            .collect::<Vec<_>>();
    }
}
