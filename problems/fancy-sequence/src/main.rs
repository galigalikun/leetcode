struct Fancy {
    data: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Fancy {

    fn new() -> Self {
        Fancy { 
            data: Vec::new(),
         }
    }
    
    fn append(&mut self, val: i32) {
        self.data.push(val);
    }
    
    fn add_all(&mut self, inc: i32) {
        self.data.iter().for_each(|x| x + inc);
    }
    
    fn mult_all(&mut self, m: i32) {
        self.data.iter().for_each(|x| x * m);
    }
    
    fn get_index(&self, idx: i32) -> i32 {
        self.data.get(idx as usize).unwrap_or(&0).clone()
    }
}

/**
 * Your Fancy object will be instantiated and called as such:
 * let obj = Fancy::new();
 * obj.append(val);
 * obj.add_all(inc);
 * obj.mult_all(m);
 * let ret_4: i32 = obj.get_index(idx);
 */
fn main() {
    let mut obj = Fancy::new();
    obj.append(2);
    obj.add_all(3);
    obj.append(7);
    obj.mult_all(2);
    assert_eq!(10, obj.get_index(0));
    obj.add_all(3);
    obj.append(10);
    obj.mult_all(2);
    assert_eq!(26, obj.get_index(0));
    assert_eq!(34, obj.get_index(1));
    assert_eq!(20, obj.get_index(2));
}
