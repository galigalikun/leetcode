fn main() {
    assert_eq!(Solution::maximum_swap(2736), 7236);
    assert_eq!(Solution::maximum_swap(9973), 9973);
}

struct Solution{}
impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut n = num;
        for i in 1..num.to_string().len() {
            println!("debug {:?}", num.to_string().chars().nth(i));
        }
        return 0;
    }
}
