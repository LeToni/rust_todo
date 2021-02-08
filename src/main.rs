fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    let mut todo = todo::Todo::new().expect("Initialisation of db failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Saved todo "),
            Err(error) => println!("An error occured: {}", error),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Todo updated and saved"),
                Err(error) => println!("An error occured: {}", error),
            },
        }
    }
}

mod todo {
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
    }
}
