use std::collections::HashMap;

type GodMap = HashMap<String, Vec<u8>>;

#[derive(Debug)]
pub struct MemoryDatabase {
    name: String,
    map: GodMap,
}

impl MemoryDatabase {
    pub fn new(db_name: &str) -> Self {
        MemoryDatabase {
            name: db_name.to_string(),
            map: GodMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
        self.map.get(key)
    }

    pub fn insert(&mut self, key: &str, val: Vec<u8>) -> Option<Vec<u8>> {
        self.map.insert(key.to_string(), val)
    }
}


// #[derive(Debug)]
// enum StorageLocation {
//     ProjectLocal,
//     Persistent,
//     MemoryOnly,
// }
//
// impl StorageLocation {
//     fn as_path(&self) -> &str {
//         match self {
//             // Using relative path so it works even if you move the folder
//             StorageLocation::ProjectLocal => "/home/Igris/RustProjects/mini_redis",
//             StorageLocation::Persistent => "/var/lib/mini_redis",
//             StorageLocation::MemoryOnly => "/dev/shm",
//         }
//     }
// }
