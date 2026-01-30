fn main() {
    assert_eq!(
        Solution::validate_coupons(
            vec![
                "SAVE20".to_string(),
                "PHARMA5".to_string(),
                "SAVE@20".to_string()
            ],
            vec![
                "restaurant".to_string(),
                "grocery".to_string(),
                "pharmacy".to_string(),
                "restaurant".to_string()
            ],
            vec![true, true, true, true]
        ),
        vec!["PHARMA5".to_string(), "SAVE20".to_string()]
    );
    assert_eq!(
        Solution::validate_coupons(
            vec![
                "GROCERY15".to_string(),
                "ELECTRONICS_50".to_string(),
                "DISCOUNT10".to_string()
            ],
            vec![
                "grocery".to_string(),
                "electronics".to_string(),
                "invalid".to_string()
            ],
            vec![false, true, true]
        ),
        vec!["ELECTRONICS_50".to_string()]
    );
}

struct Solution;
impl Solution {
    pub fn validate_coupons(
        code: Vec<String>,
        business_line: Vec<String>,
        is_active: Vec<bool>,
    ) -> Vec<String> {
        let mut valid_coupons = Vec::new();
        for i in 0..code.len() {
            if is_active[i] {
                valid_coupons.push(format!("{}-{}", code[i], business_line[i]));
            }
        }
        valid_coupons
    }
}
