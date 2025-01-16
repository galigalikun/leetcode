fn main() {
    assert_eq!(Solution::maximum_gain("cdbcbbaaabab".to_string(), 4, 5), 19);
    assert_eq!(
        Solution::maximum_gain("aabbaaxybbaabb".to_string(), 5, 4),
        20
    );
}

struct Solution;
impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let s = s.as_bytes().to_vec();
        let mut stack = vec![];
        let mut res = 0;
        let (mut x, mut y) = (x, y);
        let (mut first, mut second) = (b'a', b'b');
        if x < y {
            std::mem::swap(&mut x, &mut y);
            std::mem::swap(&mut first, &mut second);
        }
        for &c in &s {
            if c == first {
                stack.push(c);
            } else if c == second {
                if x > 0 {
                    stack.pop();
                    res += x;
                } else {
                    stack.push(c);
                }
            } else {
                let mut temp = vec![];
                while let Some(&c) = stack.last() {
                    if c == first {
                        stack.pop();
                        if y > 0 {
                            res += y;
                        } else {
                            temp.push(c);
                        }
                    } else {
                        temp.push(c);
                    }
                }
                stack.extend(temp.into_iter().rev());
            }
        }
        res
    }
}
