fn main() {
    for n in 1..=20 {
        let nums = Solution::beautiful_array(n);
        assert!(is_beautiful(&nums, n));
    }
}

struct Solution;
impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let mut ans = vec![1];

        while ans.len() < n as usize {
            let odd = ans
                .iter()
                .map(|&x| 2 * x - 1)
                .filter(|&x| x <= n)
                .collect::<Vec<i32>>();
            let even = ans
                .iter()
                .map(|&x| 2 * x)
                .filter(|&x| x <= n)
                .collect::<Vec<i32>>();

            ans = odd.into_iter().chain(even.into_iter()).collect();
        }

        ans
    }
}

fn is_beautiful(nums: &[i32], n: i32) -> bool {
    if nums.len() != n as usize {
        return false;
    }

    let mut seen = vec![false; (n + 1) as usize];
    for &x in nums {
        if x < 1 || x > n || seen[x as usize] {
            return false;
        }
        seen[x as usize] = true;
    }

    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            for k in (i + 1)..j {
                if 2 * nums[k] == nums[i] + nums[j] {
                    return false;
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beautiful_array_is_valid_for_small_n() {
        for n in 1..=50 {
            let nums = Solution::beautiful_array(n);
            assert!(is_beautiful(&nums, n));
        }
    }
}
