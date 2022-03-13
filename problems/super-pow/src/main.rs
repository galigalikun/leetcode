fn main() {
    assert_eq!(Solution::super_pow(2, vec![3]), 8);
    assert_eq!(Solution::super_pow(2, vec![1, 0]), 1024);
    assert_eq!(Solution::super_pow(1, vec![4, 3, 3, 8, 5, 2]), 1);
    assert_eq!(Solution::super_pow(2147483647, vec![2, 0, 0]), 1198);
}

struct Solution {}
// https://baihuqian.github.io/2018-08-20-super-pow/
impl Solution {
    fn pow_mod(a: i32, b: i32) -> i32 {
        if b == 0 {
            return 1;
        }
        let c = a % 1337;
        let mut result = c;
        for _i in 1..b {
            result = result * c % 1337;
        }
        return result;
    }
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        let c = a % 1337;
        if c == 0 {
            return a;
        }
        if b.len() == 0 {
            return 1;
        }
        let mut result = Solution::pow_mod(a, b[0]);
        for i in 1..b.len() {
            result = (Solution::pow_mod(result, 10) * Solution::pow_mod(c, b[i])) % 1337;
        }

        return result;
    }
}
