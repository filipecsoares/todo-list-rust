use std::collections::HashMap;

pub struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    pub fn new() -> Result<Todo, std::io::Error> {
        // open the db file or create if not exists
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;
        // serialize json as HashMap
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }

    pub fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    pub fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        // open db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
        // write to file with serde
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    pub fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }

    pub fn edit(&mut self, key: &String, new_name: String) -> Option<()> {
        if let Some(v) = self.map.remove(key) {
            self.map.insert(new_name, v);
            Some(())
        } else {
            None
        }
    }

    pub fn remove(&mut self, key: &String) -> Option<()> {
        match self.map.remove_entry(key) {
            Some((_k, _v)) => Some(()),
            None => None,
        }
    }

    pub fn show(&self) {
        for (key, value) in self.map.clone().into_iter() {
            if value {
                println!("[ ] {}", key);
            } else {
                println!("[X] {}", key);
            }
        }
    }
}