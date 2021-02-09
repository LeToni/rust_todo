use std::collections::HashMap;

pub struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    pub fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("resources/todos.json")?;
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occured: {}", e),
        }
    }

    pub fn insert(&mut self, key: String) {
        self.map.insert(key, false);
    }

    pub fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("resources/todos.json")?;
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    pub fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = true),
            None => None,
        }
    }

    pub fn delete(&mut self, key: &String) -> Option<()> {
        match self.map.remove(key) {
            Some(_) => Some(()),
            None => None,
        }
    }
}
