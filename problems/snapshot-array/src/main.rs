use std::collections::HashMap;

struct SnapshotArray {
    snap_id: usize,
    current: HashMap<i32, i32>,
    // history[i] contains only the changes made in snapshot i
    // If history[i] is empty but snap i exists, it means no changes
    history: Vec<Option<HashMap<i32, i32>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {
    fn new(_length: i32) -> Self {
        SnapshotArray {
            snap_id: 0,
            current: HashMap::new(),
            history: Vec::new(),
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.current.insert(index, val);
    }

    fn snap(&mut self) -> i32 {
        let result = self.snap_id as i32;
        
        if self.current.is_empty() {
            // No changes - don't store anything, just track the snapshot number
            self.history.push(None);
        } else {
            // Store changes and clear
            self.history.push(Some(self.current.clone()));
            self.current.clear();
        }
        
        self.snap_id += 1;
        result
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let snap_id = snap_id as usize;
        
        // Search from the requested snapshot backwards for the value
        if snap_id < self.history.len() {
            for i in (0..=snap_id).rev() {
                if let Some(ref map) = self.history[i] {
                    if let Some(&val) = map.get(&index) {
                        return val;
                    }
                }
            }
        }
        0
    }
}

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */
fn main() {
    // Test 1: Basic test from problem
    let mut obj = SnapshotArray::new(3);
    obj.set(0, 5);
    assert_eq!(obj.snap(), 0);
    obj.set(0, 6);
    assert_eq!(obj.snap(), 1);
    assert_eq!(obj.get(0, 0), 5);
    assert_eq!(obj.get(0, 1), 6);
    
    // Test 2: The failing test case
    let mut obj2 = SnapshotArray::new(1);
    obj2.set(0, 15);
    assert_eq!(obj2.snap(), 0);  // snapshot 0: {0: 15}
    assert_eq!(obj2.snap(), 1);  // snapshot 1: {0: 15} (inherits from previous)
    assert_eq!(obj2.snap(), 2);  // snapshot 2: {0: 15} (inherits from previous)
    assert_eq!(obj2.get(0, 2), 15);  // Position 5 in test case
    obj2.set(0, 2);
    assert_eq!(obj2.snap(), 3);  // snapshot 3: {0: 2}
    assert_eq!(obj2.snap(), 4);  // snapshot 4: {0: 2} (inherits from previous)
    assert_eq!(obj2.get(0, 0), 15);
    
    // Test 3: Multiple snapshots with no changes
    let mut obj3 = SnapshotArray::new(5);
    obj3.set(0, 1);
    obj3.set(1, 2);
    assert_eq!(obj3.snap(), 0);
    obj3.set(0, 3);
    assert_eq!(obj3.snap(), 1);
    assert_eq!(obj3.get(0, 0), 1);
    assert_eq!(obj3.get(1, 0), 2);
    assert_eq!(obj3.get(0, 1), 3);
    assert_eq!(obj3.get(1, 1), 2); // Inherits from snapshot 0
    
    println!("All tests passed!");
}
