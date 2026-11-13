use std::collections::HashMap;

struct SnapshotArray {
    snap_id: usize,
    current: HashMap<i32, i32>,
    history: Vec<HashMap<i32, i32>>,
}

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
        if !self.current.is_empty() {
            self.history.push(self.current.clone());
        } else if self.history.is_empty() {
            self.history.push(HashMap::new());
        } else {
            self.history.push(HashMap::new());
        }
        self.snap_id += 1;
        self.current.clear();
        result
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let snap_id = snap_id as usize;
        for i in (0..=snap_id).rev() {
            if i < self.history.len() {
                if let Some(&val) = self.history[i].get(&index) {
                    return val;
                }
            }
        }
        0
    }
}

fn main() {
    let mut obj = SnapshotArray::new(20000);
    
    // Simulate the large test: set each index 0-19999, snap after each
    for i in 0..20000 {
        obj.set(i, 368937171 + i);
        obj.snap();
    }
    
    println!("After 20000 sets and snaps");
    println!("Memory snapshots stored: {}", obj.history.len());
    
    // Test some gets
    for i in (0..100).step_by(10) {
        let val = obj.get(i, i as i32);
        println!("get({}, {}) = {}", i, i, val);
    }
    
    println!("Large test completed successfully!");
}
