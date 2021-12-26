fn main() {
    assert_eq!(Solution::next_greater_element(12), 21);
    assert_eq!(Solution::next_greater_element(21), -1);
}

struct Solution{}
impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut nums = format!("{}", n).chars().map(|x| x as i32 - 48).collect::<Vec<_>>();
        
        println!("debug {:?}", nums);
        return 0;
    }
}
