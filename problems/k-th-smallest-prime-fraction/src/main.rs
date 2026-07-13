use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

fn main() {
    assert_eq!(Solution::kth_smallest_prime_fraction(vec![1,2,3,5], 3), vec![2,5]);
    assert_eq!(Solution::kth_smallest_prime_fraction(vec![1,7], 1), vec![1,7]);
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Fraction {
    num: i32,
    den: i32,
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = self.num as i64 * other.den as i64;
        let rhs = other.num as i64 * self.den as i64;
        lhs.cmp(&rhs)
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Solution{}
impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let n = arr.len();
        let mut heap: BinaryHeap<Reverse<(Fraction, usize, usize)>> = BinaryHeap::new();

        for j in 1..n {
            heap.push(Reverse((
                Fraction {
                    num: arr[0],
                    den: arr[j],
                },
                0,
                j,
            )));
        }

        for _ in 1..k {
            let Reverse((_, i, j)) = heap.pop().expect("heap should contain at least k fractions");
            if i + 1 < j {
                heap.push(Reverse((
                    Fraction {
                        num: arr[i + 1],
                        den: arr[j],
                    },
                    i + 1,
                    j,
                )));
            }
        }

        let Reverse((_, i, j)) = heap.pop().expect("heap should contain the kth fraction");
        vec![arr[i], arr[j]]
    }
}
