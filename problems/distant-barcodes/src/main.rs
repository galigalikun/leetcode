use std::collections::HashMap;

fn main() {
    assert_eq!(
        Solution::rearrange_barcodes(vec![1, 1, 1, 2, 2, 2]),
        vec![2, 1, 2, 1, 2, 1]
    );
    assert_eq!(
        Solution::rearrange_barcodes(vec![1, 1, 1, 1, 2, 2, 3, 3]),
        vec![1, 3, 1, 3, 1, 2, 1, 2]
    );
}

struct Solution;
impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let mut counts = HashMap::new();
        for &barcode in &barcodes {
            *counts.entry(barcode).or_insert(0) += 1;
        }

        let mut items: Vec<(i32, i32)> = counts.into_iter().collect();
        items.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));

        let mut result = vec![0; barcodes.len()];
        let mut index = 0;

        for (barcode, count) in items {
            for _ in 0..count {
                result[index] = barcode;
                index += 2;
                if index >= result.len() {
                    index = 1;
                }
            }
        }

        result
    }
}
