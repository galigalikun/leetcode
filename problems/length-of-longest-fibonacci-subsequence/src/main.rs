fn main() {
    assert_eq!(
        Solution::len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8]),
        5
    );
    assert_eq!(
        Solution::len_longest_fib_subseq(vec![1, 3, 7, 11, 12, 14, 18]),
        3
    );
    assert_eq!(
        Solution::len_longest_fib_subseq(vec![2, 4, 7, 8, 9, 10, 14, 15, 18, 23, 32, 50]),
        5
    );
    assert_eq!(
        Solution::len_longest_fib_subseq(vec![
            2392, 2545, 2666, 5043, 5090, 5869, 6978, 7293, 7795
        ]),
        0
    );
}

struct Solution;
impl Solution {
    fn helper(result: &mut i32, i: usize, j: usize, a: i32, b: i32, arr: Vec<i32>) {
        if let Some(p) = arr[j..].iter().position(|&p| p == a + b) {
            *result += 1;
            Solution::helper(result, i, p, b, a + b, arr);
        }
    }
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        // [2, 4, 7, 8, 9, 10, 14, 15, 18, 23, 32, 50]
        // [4, 14, 18, 32, 50]
        let mut ans = 0;
        for i in 0..arr.len() - 1 {
            for j in i + 1..arr.len() - 2 {
                let mut a = 0;
                Solution::helper(&mut a, i, j, arr[i], arr[j], arr.clone());
                ans = std::cmp::max(ans, if a > 0 { a + 2 } else { 0 });
            }
        }
        return ans as i32;
    }
}
