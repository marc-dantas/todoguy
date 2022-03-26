use std::fs::File;
use std::io::{Write, Read};

pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: i32, title: String, completed: bool) -> Todo {
        Todo {
            id: id,
            title: title,
            completed: completed,
        }
    }
}


pub struct TodoList {
    pub items: Vec<Todo>,
}

impl TodoList {

    pub fn new() -> TodoList {
        TodoList {
            items: Vec::new(),
        }
    }

    pub fn save_to_file(&self) {
        let mut file = File::create("todos.txt").unwrap();
        for item in &self.items {
            let line = format!("{} {} {}\n", item.id, item.completed, item.title);
            file.write(line.as_bytes()).unwrap();
        }
    }

    pub fn apply_file(&mut self) {
        let mut file = File::open("todos.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        for line in contents.lines() {
            let parts: Vec<&str> = line.splitn(3, " ").collect();
            let id = parts[0].parse::<i32>().unwrap();
            let completed = parts[1].parse::<bool>().unwrap();
            let title = parts[2].to_string();
            self.items.push(Todo::new(id, title, completed));
        }
    }

    pub fn insert(&mut self, title: String) {
        self.apply_file();
        let id = self.items.len() as i32;
        self.items.push(Todo::new(id, title, false));
        self.save_to_file();
    }

    pub fn remove(&mut self, id: i32) {
        self.apply_file();
        let mut index = 0;
        for item in self.items.iter() {
            if item.id == id {
                break;
            }
            index += 1;
        }
        if index < self.items.len() {
            self.items.remove(index);
        }
        self.save_to_file();
    }

    pub fn toggle(&mut self, id: i32) {
        self.apply_file();
        for item in self.items.iter_mut() {
            if item.id == id {
                item.completed = !item.completed;
            }
        }
        self.save_to_file();
    }

    pub fn list(&mut self) {
        self.apply_file();
        if self.items.len() == 0 {
            println!("No todos for today! :)");
        } else {
            for item in self.items.iter() {
                if item.completed {
                    println!("{}: [X] {}", item.id, item.title);
                } else {
                    println!("{}: [ ] {}", item.id, item.title);
                }
            }
        }
        self.save_to_file();
    }
}