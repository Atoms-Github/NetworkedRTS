use std::collections::HashMap;

pub struct FileCache<T>{
    cache: HashMap<String, T>
}
impl<T> Default for FileCache<T>{
    fn default() -> Self {
        Self{
            cache: Default::default(),
        }
    }
}

impl<T> FileCache<T>{
    pub fn new() -> Self{
        Self{
            cache: Default::default()
        }
    }
    pub fn get<F: FnOnce(&[u8]) -> T>(&mut self, query: &String, load: F) -> &T{
        if self.cache.contains_key(query){
            return self.cache.get(query).unwrap();
        }

        let filename = format!("./resources/{}", query);
        let file_bytes = std::fs::read(&filename).unwrap_or_else(|e|{
            panic!("Failed to load a file. {:?}", &filename);
        });

        let parsed = (load)(&file_bytes);
        let existing = self.cache.insert(query.clone(), parsed);
        assert!(existing.is_none());

        return self.cache.get(query).unwrap();
    }
}