fn main() {
    assert_eq!(Solution::get_permutation(3, 3), "213".to_string());
    assert_eq!(Solution::get_permutation(4, 9), "2314".to_string());
    assert_eq!(Solution::get_permutation(3, 1), "123".to_string());
}

pub struct Solution {}
impl Solution {
    // http://www.cs.toronto.edu/~mhsadi/code-repository/15-PermutationSequence.html
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut kth_permutation = "".to_string();
        let mut letters = Vec::with_capacity(n as usize);
        let mut i_minus_one_factorial = 1;
        let mut k_mut = k;
        for i in 1..=n {
            letters.push(i);
            i_minus_one_factorial = i_minus_one_factorial * i;
        }

        for i in (1..n).rev() {
            i_minus_one_factorial = i_minus_one_factorial / (i + 1);
            if k_mut >= i_minus_one_factorial {
                let number = k_mut / i_minus_one_factorial;
                k_mut = k_mut % i_minus_one_factorial;
                if k_mut > 0 {
                    kth_permutation.push_str(letters.remove(number as usize).to_string().as_str());
                } else {
                    kth_permutation
                        .push_str(letters.remove(number as usize - 1).to_string().as_str());
                    break;
                }
            } else {
                kth_permutation.push_str(letters.remove(0).to_string().as_str());
            }
        }

        kth_permutation.push_str(
            letters
                .iter()
                .rev()
                .map(|x| x.to_string())
                .collect::<String>()
                .as_str(),
        );

        return kth_permutation;
    }
}
