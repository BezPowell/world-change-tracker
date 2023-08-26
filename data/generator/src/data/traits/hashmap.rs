use super::CloseMatch;
use std::{collections::HashMap, hash::Hash};

impl<K, V> CloseMatch<K> for HashMap<K, V>
where
    K: num::PrimInt + Hash,
    V: Sized,
{
    type Value = V;

    fn get_closest_match(&self, k: &K) -> Option<&V> {
        match self.get(k) {
            Some(value) => Some(value),
            None => {
                let keys: Vec<K> = self.keys().map(|x| *x).collect();

                match keys.get_closest_match(k) {
                    Some(key) => self.get(key),
                    None => None,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::traits::CloseMatch;
    use std::collections::HashMap;

    #[test]
    fn get_exact_match() {
        let mut data = HashMap::new();
        data.insert(1950, 1);
        data.insert(1949, 2);
        data.insert(1951, 3);

        assert_eq!(data.get_closest_match(&1950), Some(&1));
    }

    #[test]
    fn get_close_match() {
        let mut data = HashMap::new();
        data.insert(1952, 1);
        data.insert(1949, 2);
        data.insert(1953, 3);

        assert_eq!(data.get_closest_match(&1950), Some(&2));
    }

    #[test]
    fn get_empty_match() {
        let data: HashMap<u32, u32> = HashMap::new();

        assert_eq!(data.get_closest_match(&1950), None);
    }
}
