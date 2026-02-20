fn main() {
    assert_eq!(Solution::smallest_number(5), 7);
    assert_eq!(Solution::smallest_number(10), 15);
    assert_eq!(Solution::smallest_number(3), 3);
}

struct Solution;
impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }

        // Find the number of bits needed to represent n
        let mut bits = 0;
        let mut temp = n;
        while temp > 0 {
            temp >>= 1;
            bits += 1;
        }

        // Return 2^bits - 1 (all bits set for that many positions)
        (1 << bits) - 1
    }
}
