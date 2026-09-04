struct RLEIterator {
    encoded: Vec<i64>,
    pair_index: usize,
    consumed_in_pair: i64,
}

impl RLEIterator {
    fn new(encoding: Vec<i32>) -> Self {
        Self {
            encoded: encoding.into_iter().map(|v| v as i64).collect(),
            pair_index: 0,
            consumed_in_pair: 0,
        }
    }

    fn next(&mut self, n: i32) -> i32 {
        let mut remaining = n as i64;

        while self.pair_index + 1 < self.encoded.len() {
            let count = self.encoded[self.pair_index];
            let value = self.encoded[self.pair_index + 1] as i32;
            let available = count - self.consumed_in_pair;

            if remaining > available {
                remaining -= available;
                self.pair_index += 2;
                self.consumed_in_pair = 0;
                continue;
            }

            self.consumed_in_pair += remaining;
            return value;
        }

        -1
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterates_basic_case() {
        let mut it = RLEIterator::new(vec![3, 8, 0, 9, 2, 5]);

        assert_eq!(it.next(2), 8);
        assert_eq!(it.next(1), 8);
        assert_eq!(it.next(1), 5);
        assert_eq!(it.next(2), -1);
    }

    #[test]
    fn skips_zero_runs() {
        let mut it = RLEIterator::new(vec![0, 7, 1, 4]);

        assert_eq!(it.next(1), 4);
        assert_eq!(it.next(1), -1);
    }
}
