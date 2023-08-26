use super::CloseMatch;

impl<K> CloseMatch<K> for [K]
where
    K: num::PrimInt,
{
    type Value = K;

    fn get_closest_match(&self, k: &K) -> Option<&Self::Value> {
        let mut values = Vec::with_capacity(self.len());

        // Calculate differences
        for value in self {
            let diff = if value > k { *value - *k } else { *k - *value };

            values.push((value, diff))
        }

        values.sort_by(|a, b| b.1.cmp(&a.1));

        match values.pop() {
            Some(value) => Some(value.0),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::traits::CloseMatch;

    #[test]
    fn get_exact_match() {
        let mut data = Vec::new();
        data.push(1950);
        data.push(1949);
        data.push(1951);

        assert_eq!(data.get_closest_match(&1950), Some(&1950));
    }

    #[test]
    fn get_close_match() {
        let mut data = Vec::new();
        data.push(1952);
        data.push(1949);
        data.push(1953);

        assert_eq!(data.get_closest_match(&1950), Some(&1949));
    }

    #[test]
    fn get_empty_match() {
        let data: Vec<u32> = Vec::new();

        assert_eq!(data.get_closest_match(&1950), None);
    }
}
