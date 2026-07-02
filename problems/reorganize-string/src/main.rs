use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    assert_eq!(Solution::reorganize_string("aab".to_string()), "aba");
    assert_eq!(Solution::reorganize_string("aaab".to_string()), "");
}

struct Solution {}

#[derive(Eq, PartialEq)]
struct HeapEntry {
    count: i32,
    ch: char,
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count
            .cmp(&other.count)
            .then_with(|| self.ch.cmp(&other.ch))
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut counts: HashMap<char, i32> = HashMap::new();
        for ch in s.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }

        let mut heap: BinaryHeap<HeapEntry> = BinaryHeap::new();
        for (ch, count) in counts {
            heap.push(HeapEntry { count, ch });
        }

        let mut result = String::with_capacity(s.len());

        while let Some(mut first) = heap.pop() {
            let can_use_first = result.chars().last().map_or(true, |last| last != first.ch);

            if can_use_first {
                result.push(first.ch);
                first.count -= 1;
                if first.count > 0 {
                    heap.push(first);
                }
                continue;
            }

            let Some(mut second) = heap.pop() else {
                return "".to_string();
            };

            result.push(second.ch);
            second.count -= 1;

            heap.push(first);
            if second.count > 0 {
                heap.push(second);
            }
        }

        result
    }
}
