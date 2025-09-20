fn main() {
    assert_eq!(
        Solution::kth_largest_number(
            vec![
                "3".to_string(),
                "6".to_string(),
                "7".to_string(),
                "10".to_string()
            ],
            4
        ),
        "3".to_string()
    );
    assert_eq!(
        Solution::kth_largest_number(
            vec![
                "2".to_string(),
                "21".to_string(),
                "12".to_string(),
                "1".to_string()
            ],
            3
        ),
        "2".to_string()
    );
    assert_eq!(
        Solution::kth_largest_number(vec!["0".to_string(), "0".to_string()], 2),
        "0".to_string()
    );
}

struct Solution;
impl Solution {
    pub fn kth_largest_number(nums: Vec<String>, k: i32) -> String {
        let k = k as usize;
        let mut nums = nums
            .into_iter()
            .map(|x| x.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();
        nums.sort();
        nums.reverse();
        nums[k - 1].to_string()
    }
}
