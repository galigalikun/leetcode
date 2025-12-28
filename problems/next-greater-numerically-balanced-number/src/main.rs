fn main() {
    assert_eq!(Solution::next_beautiful_number(1), 22);
    assert_eq!(Solution::next_beautiful_number(1000), 1333);
    assert_eq!(Solution::next_beautiful_number(3000), 3133);
}

struct Solution;
impl Solution {
    fn is_beautiful(num: i32) -> bool {
        let mut count = [0; 10];
        let mut n = num;
        while n > 0 {
            let digit = (n % 10) as usize;
            count[digit] += 1;
            n /= 10;
        }
        for i in 1..10 {
            if count[i] != 0 && count[i] != i as i32 {
                return false;
            }
        }
        true
    }
    pub fn next_beautiful_number(n: i32) -> i32 {
        let mut num = n + 1;
        while !Self::is_beautiful(num) {
            num += 1;
        }
        num
    }
}
