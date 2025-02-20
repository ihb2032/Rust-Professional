use serde_json::Value;
use std::collections::{HashMap, HashSet};

pub fn count_provinces() -> String {
    let json_str = std::fs::read_to_string("district.json").expect("Failed to read district.json");
    let data: Value = serde_json::from_str(&json_str).expect("Failed to parse JSON");

    let mut results = Vec::new();
    let mut batch_nums: Vec<&String> = data.as_object().unwrap().keys().collect();
    batch_nums.sort();

    for batch_num in batch_nums {
        if let Some(batch) = data[batch_num].as_object() {
            let mut uf = UnionFind::new();

            for (item, related_items) in batch {
                if let Some(related_array) = related_items.as_array() {
                    for related in related_array {
                        if let Some(related_str) = related.as_str() {
                            uf.union(item, related_str);
                        }
                    }
                }
            }

            let components = uf.count_components();
            results.push(components.to_string());
        }
    }

    results.join(",")
}

struct UnionFind {
    parent: HashMap<String, String>,
    rank: HashMap<String, usize>,
}

impl UnionFind {
    fn new() -> Self {
        UnionFind {
            parent: HashMap::new(),
            rank: HashMap::new(),
        }
    }

    fn find(&mut self, x: &str) -> String {
        if self.parent.get(x).unwrap_or(&x.to_string()) != x {
            let parent = self.parent.get(x).unwrap().clone();
            let root = self.find(&parent);
            self.parent.insert(x.to_string(), root.clone());
            root
        } else {
            x.to_string()
        }
    }

    fn union(&mut self, x: &str, y: &str) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            let rank_x = *self.rank.get(&root_x).unwrap_or(&0);
            let rank_y = *self.rank.get(&root_y).unwrap_or(&0);

            if rank_x > rank_y {
                self.parent.insert(root_y, root_x);
            } else if rank_x < rank_y {
                self.parent.insert(root_x, root_y);
            } else {
                self.parent.insert(root_y.clone(), root_x.clone());
                self.rank.insert(root_x, rank_x + 1);
            }
        }
    }

    fn count_components(&mut self) -> usize {
        let keys: Vec<String> = self.parent.keys().cloned().collect();
        let mut unique_roots = HashSet::new();

        for key in keys {
            let root = self.find(&key);
            unique_roots.insert(root);
        }

        unique_roots.len()
    }
}
