fn main() {
    assert_eq!(Solution::largest_number(vec![10, 2]), "210".to_string());
    assert_eq!(
        Solution::largest_number(vec![3, 30, 34, 5, 9]),
        "9534330".to_string()
    );
    assert_eq!(Solution::largest_number(vec![1]), "1".to_string());
    assert_eq!(Solution::largest_number(vec![10]), "10".to_string());
    assert_eq!(
        Solution::largest_number(vec![432, 43243]),
        "43243432".to_string()
    );
    assert_eq!(
        Solution::largest_number(vec![111311, 1113]),
        "1113111311".to_string()
    );
    assert_eq!(
        Solution::largest_number(vec![999999991, 9]),
        "9999999991".to_string()
    );
    assert_eq!(Solution::largest_number(vec![0, 0]), "0".to_string());
    assert_eq!(
        Solution::largest_number(vec![999999998, 999999997, 999999999]),
        "999999999999999998999999997".to_string()
    )
}

struct Solution {}
// https://lenchen.medium.com/leetcode-179-largest-number-c71272228f2e
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut m_nums = nums;
        m_nums.sort_by(|x, y| {
            let left = format!("{}{}", x, y).parse::<i64>().unwrap();
            let right = format!("{}{}", y, x).parse::<i64>().unwrap();
            right.cmp(&left)
        });

        let s = m_nums.iter().map(|x| x.to_string()).collect::<String>();

        if &s[0..1] == "0" {
            return "0".to_string();
        }
        return s;
    }
}
