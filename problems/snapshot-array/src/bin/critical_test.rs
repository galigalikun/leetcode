use std::collections::HashMap;

struct SnapshotArray {
    snap_id: usize,
    current: HashMap<i32, i32>,
    history: Vec<Option<HashMap<i32, i32>>>,
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
        if self.current.is_empty() {
            self.history.push(None);
        } else {
            self.history.push(Some(self.current.clone()));
            self.current.clear();
        }
        self.snap_id += 1;
        result
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let snap_id = snap_id as usize;
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

fn main() {
    // Critical test: 20,000 sets then 20,000 snaps (no changes between snaps)
    let mut obj = SnapshotArray::new(20000);
    
    // Set all 20,000 indices
    for i in 0..20000 {
        obj.set(i, 368937171 + i);
    }
    println!("Completed 20,000 sets");
    
    // Take 20,000 snapshots with NO changes
    for snap_num in 0..20000 {
        let snap_id = obj.snap();
        if snap_num % 5000 == 0 {
            println!("Snap #{} returned snap_id={}", snap_num, snap_id);
        }
    }
    println!("Completed 20,000 snaps");
    
    // Test getting values from various snapshots
    for snap_id in [0, 100, 1000, 5000, 10000, 19999] {
        let val0 = obj.get(0, snap_id);
        let val100 = obj.get(100, snap_id);
        let val19999 = obj.get(19999, snap_id);
        println!("Snapshot {}: get(0)={}, get(100)={}, get(19999)={}", 
                 snap_id, val0, val100, val19999);
    }
    
    println!("\nCritical test completed successfully!");
    println!("Memory snapshots stored: {}", obj.history.len());
    let some_count = obj.history.iter().filter(|h| h.is_some()).count();
    let none_count = obj.history.len() - some_count;
    println!("Snapshots with data: {}, Snapshots without data: {}", some_count, none_count);
}
