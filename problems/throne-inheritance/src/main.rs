use std::collections::HashMap;
struct ThroneInheritance {
    data: HashMap<String, Vec<String>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ThroneInheritance {

    fn new(kingName: String) -> Self {
        let mut a = ThroneInheritance {
            data: HashMap::new(),
        };
        a.data.insert(kingName, vec![]);
        a

    }
    
    fn birth(&mut self, parent_name: String, child_name: String) {
        let mut v = self.data.get(&parent_name).unwrap().clone();
        v.push(child_name);
        self.data.insert(parent_name, v);
    }
    
    fn death(&mut self, name: String) {
        self.data.remove(&name);
    }
    
    fn get_inheritance_order(&self) -> Vec<String> {
        self.data.keys().map(|x| x.clone()).collect()
    }
}

/**
 * Your ThroneInheritance object will be instantiated and called as such:
 * let obj = ThroneInheritance::new(kingName);
 * obj.birth(parentName, childName);
 * obj.death(name);
 * let ret_3: Vec<String> = obj.get_inheritance_order();
 */
fn main() {
    let mut obj = ThroneInheritance::new("king".to_string());
    obj.birth("king".to_string(), "andy".to_string());
    obj.birth("king".to_string(), "bob".to_string());
    obj.birth("andy".to_string(), "catherine".to_string());
    obj.birth("andy".to_string(), "matthew".to_string());
    obj.birth("bob".to_string(), "alex".to_string());
    obj.birth("bob".to_string(), "asha".to_string());
    assert_eq!(obj.get_inheritance_order(), vec!["king", "andy", "matthew", "bob", "alex", "asha", "catherine"]);
    obj.death("bob".to_string());
    assert_eq!(obj.get_inheritance_order(), vec!["king", "andy", "matthew", "alex", "asha", "catherine"]);
}
