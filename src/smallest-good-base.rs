fn main() {
    assert_eq!(Solution::smallest_good_base("13".to_string()), "3");
    assert_eq!(Solution::smallest_good_base("4681".to_string()), "8");
    assert_eq!(
        Solution::smallest_good_base("1000000000000000000".to_string()),
        "999999999999999999"
    );
}

pub struct Solution {}
// https://www.tutorialcup.com/interview/string/smallest-good-base.htm
impl Solution {
    fn helper(d: i64, n: i64) -> i64 {
        let mut out = 1;
        let mut k = 1;
        for _i in 0..n {
            k *= d;
            out += k;
        }
        return out;
    }
    pub fn smallest_good_base(n: String) -> String {
        let num = n.parse::<i64>().unwrap();
        let mut result = vec![];
        result.push(num - 1);
        for i in 2..63 {
            let v = ((num - 1) as f64).powf(1_f64 / i as f64) as i64;
            let mut j = 0;
            while j > -3 && v + j > 1 {
                let d = v + j;
                if (num - 1) % d == 0 {
                    let poly = Solution::helper(d, i);
                    if poly == num {
                        result.push(d);
                    }
                }
                j -= 1;
            }
        }
        return format!("{}", result.last().unwrap());
    }
}
