use console::*;
use rusqlite;

pub fn setup_database(path: &str) -> rusqlite::Connection {
    let db = rusqlite::Connection::open(path).unwrap();
    let query = "CREATE TABLE IF NOT EXISTS todo (
        id INTEGER PRIMARY KEY NOT NULL,
        value VARCHAR(255) NOT NULL,
        state INTEGER NOT NULL
    );";
    db.execute(query, ()).unwrap();
    db
}

pub fn getch() -> Option<char> {
    if let Ok(c) = Term::stdout().read_char() {
        return Some(c);
    }
    None
}

pub fn getline() -> Option<String> {
    if let Ok(c) = Term::stdout().read_line() {
        return Some(c);
    }
    None
}

pub struct Todo {
    id: usize,
    value: String,
    state: bool,
}

impl Todo {
    pub fn new(
        c: &rusqlite::Connection,
        value: String,
        state: bool,
    ) -> Result<Self, rusqlite::Error> {
        c.execute(
            "INSERT INTO todo (value, state) VALUES (?1, ?2)",
            (&value, state),
        )?;
        Ok(Self {
            id: c.last_insert_rowid() as usize,
            value,
            state,
        })
    }

    pub fn load(c: &rusqlite::Connection) -> Vec<Todo> {
        let mut q = c.prepare("SELECT id, value, state FROM todo").unwrap();
        let todos = q
            .query_map([], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    value: row.get(1)?,
                    state: row.get(2)?,
                })
            })
            .unwrap();
        return todos.map(|x| x.unwrap()).collect();
    }

    pub fn delete(c: &rusqlite::Connection, list: &Vec<Todo>, selection: usize) {
        let id = list.iter().nth(selection).unwrap().id;
        c.execute("DELETE FROM todo WHERE id = ?1", (id,)).unwrap();
    }

    pub fn toggle(c: &rusqlite::Connection, list: &Vec<Todo>, selection: usize) {
        let todo = list.iter().nth(selection).unwrap();
        c.execute(
            "UPDATE todo SET state = ?1 WHERE id = ?2",
            (!todo.state, todo.id),
        )
        .unwrap();
    }

    pub fn update_list(c: &rusqlite::Connection, target: &mut Vec<Todo>) {
        target.clear();
        target.extend(Todo::load(c));
    }

    pub fn list(c: &rusqlite::Connection, selection: usize) -> usize {
        let mut count = 0;
        let todos = Todo::load(c);
        if todos.len() <= 0 {
            println!("   (empty)");
            println!(
                "   {}: press [{}] to create a new todo.",
                style("HELP").bold().cyan(),
                style("N").bold().red()
            );
            println!(
                "   {}: press [{}] to show all commands.",
                style("HELP").bold().cyan(),
                style("H").bold().red()
            );
        }
        for (i, x) in todos.iter().enumerate() {
            count += 1;
            match x.state {
                true if i == selection => {
                    println!(
                        "    {} {}",
                        style("✓").bold().green(),
                        style(&x.value).bold().underlined().cyan()
                    );
                }
                false if i == selection => {
                    println!(
                        "    {} {}",
                        style("✕").bold().red(),
                        style(&x.value).bold().underlined().cyan()
                    );
                }
                true if i != selection => {
                    println!(
                        "    {} {}",
                        style("✓").bold().green(),
                        style(&x.value).cyan()
                    );
                }
                false if i != selection => {
                    println!("    {} {}", style("✕").bold().red(), style(&x.value).cyan());
                }
                _ => {}
            }
        }
        return count;
    }
}
