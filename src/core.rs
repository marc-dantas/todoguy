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

    pub fn insert(&mut self, title: String) {
        let id = self.items.len() as i32;
        self.items.push(Todo::new(id, title, false));
    }

    pub fn remove(&mut self, id: i32) {
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
    }

    pub fn toggle(&mut self, id: i32) {
        for item in self.items.iter_mut() {
            if item.id == id {
                item.completed = !item.completed;
            }
        }
    }

    pub fn list(&self) {
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
    }
}