struct ProductOfNumbers {
    nums: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {

    fn new() -> Self {
        ProductOfNumbers {
            nums: vec![1],
        }
    }
    
    fn add(&mut self, num: i32) {
        if num == 0 {
            self.nums = vec![1];
        } else {
            self.nums.push(self.nums.last().unwrap() * num);
        }
    }
    
    fn get_product(&self, k: i32) -> i32 {
        self.nums.last().unwrap() / self.nums[self.nums.len() - k as usize - 1]
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */
fn main() {
    let mut obj = ProductOfNumbers::new();
    obj.add(3);
    obj.add(0);
    obj.add(2);
    obj.add(5);
    obj.add(4);
    let ret_1: i32 = obj.get_product(2);
    assert_eq!(ret_1, 20);
    let ret_2: i32 = obj.get_product(3);
    assert_eq!(ret_2, 40);
    let ret_3: i32 = obj.get_product(4);
    assert_eq!(ret_3, 0);
    obj.add(8);
    let ret_2: i32 = obj.get_product(2);
    assert_eq!(ret_2, 32);

}
