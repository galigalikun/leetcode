fn main() {
    assert_eq!(
        Solution::reformat_date("20th Oct 2052".to_string()),
        "2052-10-20"
    );
    assert_eq!(
        Solution::reformat_date("6th Jun 1933".to_string()),
        "1933-06-06"
    );
    assert_eq!(
        Solution::reformat_date("26th May 1960".to_string()),
        "1960-05-26"
    );
}

struct Solution;
impl Solution {
    pub fn reformat_date(date: String) -> String {
        return date
            .split_whitespace()
            .map(|x| match x {
                "1st" | "2nd" | "3rd" | "21st" | "22nd" | "23rd" | "31st" => {
                    x.chars().take(2).collect::<String>()
                }
                _ => x.chars().take(1).collect::<String>(),
            })
            .collect::<Vec<String>>()
            .join(" ");
    }
}
