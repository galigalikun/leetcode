fn main() {
    assert_eq!(Solution::gcd_sort(vec![7, 21, 3]), true);
    assert_eq!(Solution::gcd_sort(vec![5, 2, 6, 2]), false);
    assert_eq!(Solution::gcd_sort(vec![10, 5, 9, 3, 15]), true);
}

struct Solution;
impl Solution {
    pub fn gcd_sort(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let nums = nums;
        let mut sorted = nums.clone();
        sorted.sort_unstable();
        let max = *nums.iter().max().unwrap() as usize;
        let mut spf = vec![0; max + 1];
        for i in 2..=max {
            if spf[i] == 0 {
                for j in (i..=max).step_by(i) {
                    if spf[j] == 0 {
                        spf[j] = i;
                    }
                }
            }
        }
        let mut uf = (0..n).collect::<Vec<_>>();
        fn find(uf: &mut Vec<usize>, x: usize) -> usize {
            if uf[x] != x {
                uf[x] = find(uf, uf[x]);
            }
            uf[x]
        }
        fn union(uf: &mut Vec<usize>, x: usize, y: usize) {
            let px = find(uf, x);
            let py = find(uf, y);
            if px != py {
                uf[px] = py;
            }
        }
        let mut prime_to_index = std::collections::HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let mut x = num as usize;
            while x > 1 {
                let p = spf[x];
                prime_to_index.entry(p).or_insert(i);
                union(&mut uf, i, *prime_to_index.get(&p).unwrap());
                while x % p == 0 {
                    x /= p;
                }
            }
        }
        for i in 0..n {
            if find(&mut uf, i) != find(&mut uf, sorted.iter().position(|&x| x == nums[i]).unwrap())
            {
                return false;
            }
        }
        return true;
    }
}
