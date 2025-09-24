struct LockingTree {
    parent: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LockingTree {

    fn new(parent: Vec<i32>) -> Self {
        LockingTree {
            parent,
        }
    }

    fn lock(&mut self, num: i32, user: i32) -> bool {
        if self.parent[num as usize] == -1 {
            return false;
        }
        self.parent[num as usize] = -1;
        self.parent[num as usize] = user;
        true
    }

    fn unlock(&mut self, num: i32, user: i32) -> bool {
        if self.parent[num as usize] == -1 {
            return false;
        }
        self.parent[num as usize] = user;
        true
    }

    fn upgrade(&mut self, num: i32, user: i32) -> bool {
        if self.parent[num as usize] == -1 {
            return false;
        }
        self.parent[num as usize] = -1;
        self.parent[num as usize] = user;

        true
    }
}

/**
 * Your LockingTree object will be instantiated and called as such:
 * let obj = LockingTree::new(parent);
 * let ret_1: bool = obj.lock(num, user);
 * let ret_2: bool = obj.unlock(num, user);
 * let ret_3: bool = obj.upgrade(num, user);
 */
fn main() {
    let mut obj = LockingTree::new(vec![-1, 0, 0, 1, 1, 2, 2]);
    let ret_1: bool = obj.lock(2, 2);
    assert!(ret_1);
    let ret_2: bool = obj.unlock(2, 3);
    assert!(!ret_2);
    let ret_3: bool = obj.unlock(2, 2);
    assert!(ret_3);
    let ret_4: bool = obj.lock(4, 5);
    assert!(ret_4);
    let ret_5: bool = obj.upgrade(0, 1);
    assert!(ret_5);
    let ret_6: bool = obj.lock(0, 1);
    assert!(!ret_6);
}
