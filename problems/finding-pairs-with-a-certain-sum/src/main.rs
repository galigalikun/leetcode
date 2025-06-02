struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    count_map: std::collections::HashMap<i32, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {

    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        FindSumPairs{
            nums1: nums1,
            nums2: nums2,
            count_map: std::collections::HashMap::new(),
        }
    }

    fn add(&mut self, index: i32, val: i32) {
        let mut nums2 = self.nums2.clone();
        if let Some(x) = nums2.get_mut(index as usize) {
            *x += val;
        }
        // Update the count_map based on the new value
        for &num1 in &self.nums1 {
            let sum = num1 + nums2[index as usize];
            *self.count_map.entry(sum).or_insert(0) += 1;
        }
        // Update the nums2 in the struct
        self.nums2 = nums2;
        // Rebuild the count_map
        self.count_map.clear();
        for &num1 in &self.nums1 {
            for &num2 in &self.nums2 {
                let sum = num1 + num2;
                *self.count_map.entry(sum).or_insert(0) += 1;
            }
        }
    }

    fn count(&self, tot: i32) -> i32 {
        // Count the number of pairs that sum to `tot`
        self.count_map.get(&tot).cloned().unwrap_or(0)
    }
}

/**
 * Your FindSumPairs object will be instantiated and called as such:
 * let obj = FindSumPairs::new(nums1, nums2);
 * obj.add(index, val);
 * let ret_2: i32 = obj.count(tot);
 */
fn main() {
    let mut obj = FindSumPairs::new(vec![1, 1, 2, 2, 2, 3], vec![1, 4, 5, 2, 5, 4]);
    assert_eq!(obj.count(7), 8);
    obj.add(3, 2);
    assert_eq!(obj.count(8), 2);
    assert_eq!(obj.count(4), 1);
    obj.add(0, 1);
    obj.add(1, 1);
    assert_eq!(obj.count(7), 11);
}
