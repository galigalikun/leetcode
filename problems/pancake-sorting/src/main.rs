fn main() {
    assert_sorted_by_flips(vec![3, 2, 4, 1]);
    assert_sorted_by_flips(vec![1, 2, 3]);
}

struct Solution;
impl Solution {
    pub fn pancake_sort(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        let n = arr.len();
        let mut flips = Vec::new();

        for size in (2..=n).rev() {
            let mut max_idx = 0;
            for i in 1..size {
                if arr[i] > arr[max_idx] {
                    max_idx = i;
                }
            }

            if max_idx == size - 1 {
                continue;
            }

            if max_idx != 0 {
                arr[..=max_idx].reverse();
                flips.push((max_idx + 1) as i32);
            }

            arr[..size].reverse();
            flips.push(size as i32);
        }

        flips
    }
}

fn assert_sorted_by_flips(input: Vec<i32>) {
    let flips = Solution::pancake_sort(input.clone());
    assert!(flips.len() <= 10 * input.len());

    let mut actual = input;
    for &k in &flips {
        let k = k as usize;
        assert!(k >= 1 && k <= actual.len());
        actual[..k].reverse();
    }

    let mut expected = actual.clone();
    expected.sort();
    assert_eq!(actual, expected);
}
