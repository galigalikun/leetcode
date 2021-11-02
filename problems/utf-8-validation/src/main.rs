fn main() {
    assert_eq!(Solution::valid_utf8(vec![197, 130, 1]), true);
    assert_eq!(Solution::valid_utf8(vec![235, 140, 4]), false);
    assert_eq!(Solution::valid_utf8(vec![230, 136, 145]), true);
}

pub struct Solution {}
impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut a: Option<i32> = None;
        for b in data {
            let s = format!("{:0>8b}", b);
            if a == None {
                if &s[0..1] == "0" {
                    a = None;
                } else if &s[0..3] == "110" {
                    a = Some(2);
                } else if &s[0..4] == "1110" {
                    a = Some(3);
                } else if &s[0..5] == "11110" {
                    a = Some(4);
                } else {
                    return false;
                }
            } else if a == Some(4) {
                if &s[0..2] == "10" {
                    a = Some(3);
                } else {
                    return false;
                }
            } else if a == Some(3) {
                if &s[0..2] == "10" {
                    a = Some(2);
                } else {
                    return false;
                }
            } else if a == Some(2) {
                if &s[0..2] == "10" {
                    a = None;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        return a == None;
    }
}
