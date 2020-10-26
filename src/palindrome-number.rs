fn main () {
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(-121), false);
    assert_eq!(Solution::is_palindrome(10), false);
    assert_eq!(Solution::is_palindrome(-101), false);
    assert_eq!(Solution::is_palindrome(0), true);
    assert_eq!(Solution::is_palindrome(1410110141), true);
}

pub struct Solution {}
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x >= 0 {
            let mut i = 0;
            let mut n = x/(10 as i32).pow(i);
            let mut a = Vec::new();
            while n > 0 {
                a.push(n%10);
                i+=1;
                n = ((x as i64)/(10 as i64).pow(i)) as i32;
            }
            i = (a.len()) as u32;
            let mut num = 0;
            for d in a {
                i-=1;
                num +=d*(10 as i32).pow(i);
            }
            return x == num;
        }
        return false;
    }
}
