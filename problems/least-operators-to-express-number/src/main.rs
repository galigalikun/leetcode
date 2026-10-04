fn main() {
    assert_eq!(Solution::least_ops_express_target(3, 19), 5);
    assert_eq!(Solution::least_ops_express_target(5, 501), 8);
    assert_eq!(Solution::least_ops_express_target(100, 100000000), 3);
}

struct Solution;
impl Solution {
    pub fn least_ops_express_target(x: i32, target: i32) -> i32 {
        let x = x as i64;
        let mut target = target as i64;

        let mut k = 0i64;
        let mut pos = 0i64;
        let mut neg = 0i64;

        while target > 0 {
            let digit = target % x;
            target /= x;

            if k == 0 {
                pos = digit * 2;
                neg = (x - digit) * 2;
            } else {
                let pos2 = (digit * k + pos).min((digit + 1) * k + neg);
                let neg2 = ((x - digit) * k + pos).min((x - digit - 1) * k + neg);
                pos = pos2;
                neg = neg2;
            }

            k += 1;
        }

        (pos.min(k + neg) - 1) as i32
    }
}
