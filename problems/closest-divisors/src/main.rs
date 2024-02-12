fn main() {
    assert_eq!(Solution::closest_divisors(8), vec![3, 3]);
    assert_eq!(Solution::closest_divisors(123), vec![5, 25]);
    assert_eq!(Solution::closest_divisors(999), vec![40, 25]);
}

struct Solution;
impl Solution {
    fn get_divisors(num: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut i = 1;
        while i * i <= num {
            if num % i == 0 {
                res.push(i);
                res.push(num / i);
            }
            i += 1;
        }
        return res;
    }
    pub fn closest_divisors(num: i32) -> Vec<i32> {
        let res1 = Solution::get_divisors(num + 1);
        let res2 = Solution::get_divisors(num + 2);
        return if (res1[0] - res1[1]) > (res2[0] - res2[1]) {
            res1[..2].to_vec()
        } else {
            res2[..2].to_vec()
        };
    }
}
