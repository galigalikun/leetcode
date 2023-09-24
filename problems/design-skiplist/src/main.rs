struct Skiplist {
    data: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Skiplist {

    fn new() -> Self {
        Skiplist { 
            data: Vec::new(),
         }
    }
    
    fn search(&self, target: i32) -> bool {
        return self.data.contains(&target);
    }
    
    fn add(&mut self, num: i32) {
        self.data.push(num);
    }
    
    fn erase(&mut self, num: i32) -> bool {
        if self.data.contains(&num) {
            self.data.retain(|&x| x != num);
            return true;
        }
        return false;
    }
}

/**
 * Your Skiplist object will be instantiated and called as such:
 * let obj = Skiplist::new();
 * let ret_1: bool = obj.search(target);
 * obj.add(num);
 * let ret_3: bool = obj.erase(num);
 */
fn main() {
    let mut obj = Skiplist::new();
    obj.add(1);
    obj.add(2);
    obj.add(3);
    assert_eq!(obj.search(0), false);
    obj.add(4);
    assert_eq!(obj.search(1), true);
    assert_eq!(obj.erase(0), false);
    assert_eq!(obj.erase(1), true);
    assert_eq!(obj.search(1), false);
}
