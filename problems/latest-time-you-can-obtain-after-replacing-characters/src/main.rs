fn main() {
    assert_eq!(Solution::find_latest_time("1?:?4".to_string()), "11:54");
    assert_eq!(Solution::find_latest_time("0?:5?".to_string()), "09:59");
}

struct Solution;
impl Solution {
    pub fn find_latest_time(s: String) -> String {
        let bytes = s.as_bytes();
        let mut res = [b'0'; 5];
        for i in 0..5 {
            res[i] = bytes[i];
        }
        if res[0] == b'?' {
            if res[1] == b'?' || res[1] <= b'3' {
                res[0] = b'2';
            } else {
                res[0] = b'1';
            }
        }
        if res[1] == b'?' {
            if res[0] == b'2' {
                res[1] = b'3';
            } else {
                res[1] = b'9';
            }
        }
        if res[3] == b'?' {
            res[3] = b'5';
        }
        if res[4] == b'?' {
            res[4] = b'9';
        }
        if res[0] <= b'2'
            && ((res[0] < b'2') || (res[1] <= b'3'))
            && res[3] <= b'5'
            && res[4] <= b'9'
        {
            return String::from_utf8(res.to_vec()).unwrap();
        }
        return String::new();
    }
}
