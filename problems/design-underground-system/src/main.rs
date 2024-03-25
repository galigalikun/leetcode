use std::collections::HashMap;

struct UndergroundSystem {
    station: HashMap<String, (i32, i32)>,   // station_name -> (total_time, count)
    customer: HashMap<i32, (String, i32)>,  // id -> (station_name, time)p
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {

    fn new() -> Self {
        UndergroundSystem {
            station: HashMap::new(),
            customer: HashMap::new(),
        }
    }
    
    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.customer.insert(id, (station_name, t));
    }
    
    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        self.station.entry(self.customer[&id].0.clone()).or_insert((0, 0)).0 += t - self.customer[&id].1;
    }
    
    fn get_average_time(&mut self, start_station: String, end_station: String) -> f64 {
        let (total_time, count) = self.station[&start_station];
        return total_time as f64 / count as f64;
    }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */
fn main() {
    let obj = UndergroundSystem::new();
    obj.check_in(45, "Leyton".to_string(), 3);
    obj.check_out(32, "Paradise".to_string(), 8);
    obj.check_in(27, "Leyton".to_string(), 10);
    obj.check_out(45, "Waterloo".to_string(), 15);
    obj.check_out(27, "Waterloo".to_string(), 20);
    obj.check_out(32, "Cambridge".to_string(), 22);
    assert_eq!(obj.get_average_time("Paradise".to_string(), "Cambridge".to_string()), 14.0);
    assert_eq!(obj.get_average_time("Leyton".to_string(), "Waterloo".to_string()), 11.0);
    obj.check_in(10, "Leyton".to_string(), 24);
    assert_eq!(obj.get_average_time("Leyton".to_string(), "Waterloo".to_string()), 11.0);
    obj.check_out(10, "Waterloo".to_string(), 38);
    assert_eq!(obj.get_average_time("Leyton".to_string(), "Waterloo".to_string()), 12.0);
}
