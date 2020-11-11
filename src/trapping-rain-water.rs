fn main() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
}

pub struct Solution {}
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..height.len() {
            let p = if i > 0 { height[i - 1] } else { 0 };
            let c = height[i];
            /*
             2 > 0
             2 > 1
            */
            let n = if height.len() > i + 1 {
                height[i + 1]
            } else {
                0
            };
            println!("debug {} {} {}", p, c, n);
            if p > c && n > c {
                if p > n {
                    result = n - c;
                } else {
                    result = p - c;
                }
            } else if p > c {
                for j in i..height.len() {
                    if height[j] >= p {
                        result = p - c;
                        break;
                    }
                }
            }
        }
        return result;
    }
}
