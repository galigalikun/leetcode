use std::collections::HashMap;

struct SnapshotArray {
    snap_id: i32,
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
        let result = self.snap_id;
        self.history.push(self.current.clone());
        self.snap_id += 1;
        self.current.clear();
        result
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        for i in (0..=snap_id as usize).rev() {
            if let Some(&val) = self.history[i].get(&index) {
                return val;
            }
        }
        0
    }
}

fn main() {
    // Simulate the stress test: 20,000 length with many set operations
    let mut obj = SnapshotArray::new(20000);
    
    // Set many values and take snapshots
    for i in 0..100 {
        for j in 0..100 {
            obj.set(j, i * 100 + j);
        }
        obj.snap();
    }
    
    // Test some get operations
    for snap_id in 0..10 {
        let val = obj.get(50, snap_id);
        println!("get(50, {}) = {}", snap_id, val);
    }
    
    println!("Stress test completed successfully!");
}
