struct Router {
    memory_limit: i32,
    packets: Vec<(i32, i32, i32)>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Router {

    fn new(memory_limit: i32) -> Self {
            Self {
                memory_limit: memory_limit,
                packets: vec![],
            }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let packet = (source, destination, timestamp);
        self.memory_limit -= 1;
        if self.memory_limit < 0 {
            self.memory_limit += 1;
            return false;
        }
        self.packets.push(packet);
        return true;
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        self.memory_limit += 1;
        if let Some(packet) = self.packets.pop() {
            return vec![packet.0, packet.1, packet.2];
        }
        return vec![];
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        let mut count = 0;
        for packet in &self.packets {
            if packet.1 == destination && packet.2 >= start_time && packet.2 <= end_time {
                count += 1;
            }
        }

        return count;
    }
}

/**
 * Your Router object will be instantiated and called as such:
 * let obj = Router::new(memoryLimit);
 * let ret_1: bool = obj.add_packet(source, destination, timestamp);
 * let ret_2: Vec<i32> = obj.forward_packet();
 * let ret_3: i32 = obj.get_count(destination, startTime, endTime);
 */
fn main() {
    let mut obj = Router::new(3);
    assert_eq!(obj.add_packet(1, 40, 90), true);
    assert_eq!(obj.add_packet(2, 5, 90), true);
    assert_eq!(obj.add_packet(1, 4, 90), false);
    assert_eq!(obj.add_packet(3, 5, 95), true);
    assert_eq!(obj.add_packet(4, 5, 105), true);
    assert_eq!(obj.forward_packet(), vec![2, 5, 90]);
    assert_eq!(obj.add_packet(5, 2, 110), true);
    assert_eq!(obj.get_count(5, 100, 110), 1);
}
