use std::collections::HashMap;

#[derive(Debug)]
pub struct BiMap<K: Eq + std::hash::Hash + Clone, V: Eq + std::hash::Hash + Clone> {
    forward: HashMap<K, V>,
    backward: HashMap<V, K>,
}

impl<K: Eq + std::hash::Hash + Clone, V: Eq + std::hash::Hash + Clone> BiMap<K, V> {
    pub fn new() -> Self {
        BiMap {
            forward: HashMap::new(),
            backward: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<(K, V)> {
        if let Some(old_v) = self.forward.insert(k.clone(), v.clone()) {
            self.backward.remove(&old_v);
        }
        if let Some(old_k) = self.backward.insert(v.clone(), k.clone()) {
            self.forward.remove(&old_k);
        }
        Some((k, v))
    }

    pub fn get_by_key(&self, k: &K) -> Option<&V> {
        self.forward.get(k)
    }

    pub fn get_by_value(&self, v: &V) -> Option<&K> {
        self.backward.get(v)
    }
}
