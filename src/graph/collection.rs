use std::hash::Hash;
use std::collections::HashMap;


#[derive(Debug)]
pub struct IndexHashMap<K>
    where K: Eq + Hash
{
    hash_map: HashMap<K, usize>,
    indices: Vec<K>,
}

impl<K> IndexHashMap<K>
    where K: Eq + Hash + Clone
{
    pub fn new() -> Self {
        Self {
            hash_map: HashMap::<K, usize>::new(),
            indices: Vec::new(),
        }
    }

    pub fn entry(&mut self, key: K) -> usize {
        let next_index = self.indices.len();

        let hash_key = key.clone();

        match self.hash_map.get(&hash_key) {
            Some(index) => *index,
            None => {
                self.indices.push(key);
                let index = self.indices.len() - 1;
                self.hash_map.insert(hash_key, index);

                index
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&usize>
    {
        self.hash_map.get(key)
    }

    pub fn contains(&self, key: &K) -> bool {
        self.hash_map.contains_key(key)
    }

    pub fn get_index(&self, index: &usize) -> Option<&K> {
        self.indices.get(*index)
    }

    pub fn contains_index(&self, index: &usize) -> bool {
        self.indices.get(*index).is_some()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creates_entries_and_uses_its_methods_for_fetching() {
        let mut index_hash_map: IndexHashMap<String> = IndexHashMap::new();

        let key_1 = "Key 1".to_string();

        assert_eq!(None, index_hash_map.get(&key_1));
        assert_eq!(false, index_hash_map.contains(&key_1));
        assert_eq!(None, index_hash_map.get_index(&0_usize));
        assert_eq!(false, index_hash_map.contains_index(&0_usize));

        assert_eq!(0_usize, index_hash_map.entry(key_1));

        // insert second key
        let key_2 = "Key 2".to_string();
        assert_eq!(1_usize, index_hash_map.entry(key_2));

        let key_1_check = "Key 1".to_string();
        // check for existence of keys
        assert_eq!(Some(&0_usize), index_hash_map.get(&key_1_check));
        assert_eq!(true, index_hash_map.contains(&key_1_check));
        assert_eq!(Some(&key_1_check), index_hash_map.get_index(&0_usize));
        assert_eq!(true, index_hash_map.contains_index(&0_usize));

        let key_2_check = "Key 2".to_string();
        // check for existence of keys
        let key_2_index_check = 1_usize;

        assert_eq!(Some(&key_2_index_check), index_hash_map.get(&key_2_check));
        assert_eq!(true, index_hash_map.contains(&key_2_check));
        assert_eq!(Some(&key_2_check), index_hash_map.get_index(&key_2_index_check));
        assert_eq!(true, index_hash_map.contains_index(&key_2_index_check));
    }
}