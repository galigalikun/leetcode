use std::ops::Add;

fn main() {
    assert_eq!(Solution::nearest_palindromic("123".to_string()), "121");
    assert_eq!(Solution::nearest_palindromic("1".to_string()), "0");
}

struct Solution{}
impl Solution {
    fn helper(n:String, dir:bool) -> String {
        let mut base = *(&n[0..n.len()-n.len()/2].parse::<i32>().unwrap());
        base += if dir {
            1
        } else {
            -1
        };
        if base == 0 {
            if n.len() == 0 {
                return "0".to_string();
            }
            return "9".to_string();
        }
        let mut left = base.to_string();
        let mut right = left.chars().rev().collect::<String>();
        if n.len() > left.len() {
            right.add("9");
        }
        left.add(&right[right.len()-n.len()..]);
        return left.clone();
    }
    pub fn nearest_palindromic(n: String) -> String {
        let p = n.chars().rev().collect::<String>();
        Solution::helper(p, false);
        return n;
    }
}
