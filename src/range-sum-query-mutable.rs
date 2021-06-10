struct NumArray {
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        NumArray { nums: nums }
    }

    fn update(&mut self, index: i32, val: i32) {
        self.nums[index as usize] = val;
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        return *(&self.nums[left as usize..=right as usize]
            .iter()
            .fold(0, |sum, a| sum + a));
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */
fn main() {
    let mut obj = NumArray::new(vec![1, 3, 5]);
    assert_eq!(obj.sum_range(0, 2), 9);
    obj.update(1, 2);
    assert_eq!(obj.sum_range(0, 2), 8);
}
