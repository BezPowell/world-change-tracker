pub mod hashmap;
pub mod vec;

pub trait CloseMatch<K> {
    type Value;

    fn get_closest_match(&self, k: &K) -> Option<&Self::Value>;
}
