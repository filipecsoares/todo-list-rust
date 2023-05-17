use std::collections::HashMap;
use std::io;

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
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

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        // open db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
        // write to file with serde
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }

    fn edit(&mut self, key: &String, new_name: String) -> Option<()> {
        if let Some(v) = self.map.remove(key) {
            self.map.insert(new_name, v);
            Some(())
        } else {
            None
        }
    }

    fn remove(&mut self, key: &String) -> Option<()> {
        match self.map.remove_entry(key) {
            Some((_k, _v)) => Some(()),
            None => None,
        }
    }

    fn show(&self) {
        for (k, v) in self.map.clone().into_iter() {
            if v {
                println!("[ ] {}", k);
            } else {
                println!("[X] {}", k);
            }
        }
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action.");
    let mut todo = Todo::new().expect("Initialization of db failed!");
    if action == "add" {
        let item = read_task_name();
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved!"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        let item = read_task_name();
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    } else if action == "show" {
        todo.show();
    } else if action == "remove" {
        let item = read_task_name();
        match todo.remove(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("item removed"),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    } else if action == "edit" {
        print!("To Edit ");
        let item = read_task_name();
        print!("Edit to ");
        let new_name = read_task_name();
        match todo.edit(&item, new_name) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("item edited"),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    }
}

fn read_task_name() -> String {
    let mut item = String::new();
    let stdin = io::stdin();
    println!("Task name:");
    stdin.read_line(&mut item).expect("Error on reading the input.");
    return item;
}
