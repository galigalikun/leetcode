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
        let mut barcodes = barcodes;
        barcodes.sort();
        let mut result = vec![];
        let mut i = 0;
        let mut j = barcodes.len() / 2;
        while i < barcodes.len() / 2 {
            result.push(barcodes[j]);
            result.push(barcodes[i]);
            i += 1;
            j += 1;
        }
        if barcodes.len() % 2 == 1 {
            result.push(barcodes[j]);
        }
        return result;
    }
}
