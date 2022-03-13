fn main() {
    assert_eq!(
        Solution::is_valid_serialization("9,3,4,#,#,1,#,#,2,#,6,#,#".to_string()),
        true
    );

    assert_eq!(Solution::is_valid_serialization("1,#".to_string()), false);

    assert_eq!(
        Solution::is_valid_serialization("9,#,#,1".to_string()),
        false
    );

    assert_eq!(
        Solution::is_valid_serialization("#,7,6,9,#,#,#".to_string()),
        false
    );
}

struct Solution {}
// https://techlarry.github.io/Leetcode/331.%20Verify%20Preorder%20Serialization%20of%20a%20Binary%20Tree/
impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut st = vec![];
        for p in preorder.split(",") {
            while p == "#" && !st.is_empty() && st.last() == Some(&p) {
                st.pop();
                if st.is_empty() {
                    return false;
                }
                st.pop();
            }
            st.push(p);
        }
        return st.len() == 1 && st.last() == Some(&"#");
    }
}
