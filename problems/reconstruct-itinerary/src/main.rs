fn main() {
    assert_eq!(
        Solution::find_itinerary(vec![
            vec!["MUC".to_string(), "LHR".to_string()],
            vec!["JFK".to_string(), "MUC".to_string()],
            vec!["SFO".to_string(), "SJC".to_string()],
            vec!["LHR".to_string(), "SFO".to_string()]
        ]),
        vec![
            "JFK".to_string(),
            "MUC".to_string(),
            "LHR".to_string(),
            "SFO".to_string(),
            "SJC".to_string()
        ]
    );

    assert_eq!(
        Solution::find_itinerary(vec![
            vec!["JFK".to_string(), "SFO".to_string()],
            vec!["JFK".to_string(), "ATL".to_string()],
            vec!["SFO".to_string(), "ATL".to_string()],
            vec!["ATL".to_string(), "JFK".to_string()],
            vec!["ATL".to_string(), "SFO".to_string()]
        ]),
        vec![
            "JFK".to_string(),
            "ATL".to_string(),
            "JFK".to_string(),
            "SFO".to_string(),
            "ATL".to_string(),
            "SFO".to_string()
        ]
    );
}

pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    fn visit(result: &mut Vec<String>, graph: &mut HashMap<String, Vec<String>>, airport: String) {
        while let Some(g) = graph.get_mut(&airport) {
            (*g).sort();
            let x = (*g).remove(0);
            if g.len() == 0 {
                graph.remove(&airport);
            }
            Solution::visit(result, graph, x);
        }
        result.push(airport);
    }
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        for t in tickets {
            let u = t.first().unwrap().to_string();
            let v = t.last().unwrap().to_string();
            if let Some(g) = graph.get_mut(&u) {
                (*g).push(v);
            } else {
                graph.insert(u, vec![v]);
            }
        }

        let mut result: Vec<String> = vec![];
        Solution::visit(&mut result, &mut graph, "JFK".to_string());

        return result.into_iter().rev().collect::<Vec<_>>();
    }
}
