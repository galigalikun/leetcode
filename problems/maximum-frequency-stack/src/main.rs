use std::collections::HashMap;

struct FreqStack {
    freq: HashMap<i32, i32>,
    groups: HashMap<i32, Vec<i32>>,
    max_freq: i32,
}

impl FreqStack {
    fn new() -> Self {
        Self {
            freq: HashMap::new(),
            groups: HashMap::new(),
            max_freq: 0,
        }
    }

    fn push(&mut self, val: i32) {
        let next_freq = self.freq.get(&val).copied().unwrap_or(0) + 1;
        self.freq.insert(val, next_freq);

        self.groups.entry(next_freq).or_default().push(val);
        self.max_freq = self.max_freq.max(next_freq);
    }

    fn pop(&mut self) -> i32 {
        let group = self
            .groups
            .get_mut(&self.max_freq)
            .expect("pop is called only when stack is not empty");

        let val = group
            .pop()
            .expect("max frequency group is non-empty when popping");

        if group.is_empty() {
            self.groups.remove(&self.max_freq);
            self.max_freq -= 1;
        }

        if let Some(current_freq) = self.freq.get_mut(&val) {
            *current_freq -= 1;
            if *current_freq == 0 {
                self.freq.remove(&val);
            }
        }

        val
    }
}

fn main() {
    let mut obj = FreqStack::new();
    obj.push(5);
    obj.push(7);
    obj.push(5);
    obj.push(7);
    obj.push(4);
    obj.push(5);

    assert_eq!(5, obj.pop());
    assert_eq!(7, obj.pop());
    assert_eq!(5, obj.pop());
    assert_eq!(4, obj.pop());
}
