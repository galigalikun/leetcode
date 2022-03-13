fn main() {
    assert_eq!(Solution::number_to_words(123), "One Hundred Twenty Three");
    assert_eq!(
        Solution::number_to_words(12345),
        "Twelve Thousand Three Hundred Forty Five"
    );
    assert_eq!(
        Solution::number_to_words(1234567),
        "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
    );
    assert_eq!(Solution::number_to_words(1234567891), "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One");
}

struct Solution {}
// https://bohenan.gitbooks.io/leetcode/content/mathstring/237-integer-to-english-words.html
const BELOW_TEN: &[&str] = &[
    "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
];
const BELOW_TWENTY: &[&str] = &[
    "Ten",
    "Eleven",
    "Twelve",
    "Thirteen",
    "Fourteen",
    "Fifteen",
    "Sixteen",
    "Seventeen",
    "Eighteen",
    "Nineteen",
];
const BELOW_HUNDRED: &[&str] = &[
    "", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
];
impl Solution {
    fn helper(num: i32) -> String {
        return if num < 10 {
            format!("{}", BELOW_TEN[num as usize])
        } else if num < 20 {
            format!("{}", BELOW_TWENTY[(num - 10) as usize])
        } else if num < 100 {
            format!(
                "{} {}",
                BELOW_HUNDRED[(num / 10) as usize],
                Solution::helper(num % 10)
            )
        } else if num < 1000 {
            format!(
                "{} Hundred {}",
                Solution::helper(num / 100),
                Solution::helper(num % 100)
            )
        } else if num < 1000000 {
            format!(
                "{} Thousand {}",
                Solution::helper(num / 1000),
                Solution::helper(num % 1000)
            )
        } else if num < 1000000000 {
            format!(
                "{} Million {}",
                Solution::helper(num / 1000000),
                Solution::helper(num % 1000000)
            )
        } else {
            format!(
                "{} Billion {}",
                Solution::helper(num / 1000000000),
                Solution::helper(num % 1000000000)
            )
        }
        .trim()
        .to_string();
    }
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        return Solution::helper(num);
    }
}
