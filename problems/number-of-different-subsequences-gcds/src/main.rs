fn main() {
    assert_eq!(
        Solution::count_different_subsequence_gc_ds(vec![6, 10, 3]),
        5
    );
    assert_eq!(
        Solution::count_different_subsequence_gc_ds(vec![5, 15, 40, 5, 6]),
        7
    );
}

struct Solution;
impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 { a } else { Self::gcd(b, a % b) }
    }
    fn is_gc(num: i32) -> bool {
        let mut sum = 0;
        let mut product = 1;
        let mut n = num;

        while n > 0 {
            let digit = n % 10;
            sum += digit;
            product *= digit;
            n /= 10;
        }

        Self::gcd(sum, product) == 1
    }
    pub fn count_different_subsequence_gc_ds(nums: Vec<i32>) -> i32 {
        let mut max_num = 0;
        for &num in &nums {
            if num > max_num {
                max_num = num;
            }
        }

        let mut gc_count = vec![0; (max_num + 1) as usize];
        for &num in &nums {
            gc_count[num as usize] += 1;
        }

        let mut result = 0;
        for i in 1..=max_num {
            if gc_count[i as usize] == 0 {
                continue;
            }
            if Self::is_gc(i) {
                result += 1;
            }
        }

        result
    }
}
