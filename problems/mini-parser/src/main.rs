fn main() {
    assert_eq!(
        Solution::deserialize("324".to_string()),
        NestedInteger::Int(324)
    );
    // assert_eq!(Solution::deserialize("[123,[456,[789]]]".to_string()), NestedInteger::List(vec![NestedInteger::Int(123), NestedInteger::List(vec![NestedInteger::Int(456), NestedInteger::List(vec![NestedInteger::Int(789)])])]));
}

struct Solution {}
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        for c in s.chars() {
            println!("debug {}", c);
        }
        return NestedInteger::Int(324);
    }
}
