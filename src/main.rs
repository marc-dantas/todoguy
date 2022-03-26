// Author: @marc-dantas
// <https://github.com/marc-dantas>
mod shell;
mod core;

fn display_todos(todo_list: &core::TodoList) {
    println!("------ Todos ------");
    todo_list.list();
    println!("-------------------");
}

fn ask_id() -> i32 {
    let mut id = String::new();
    println!("\nEnter the id of the todo you want to use:");
    std::io::stdin().read_line(&mut id).expect("Failed to read line");
    id.trim().parse::<i32>().expect("Please enter a number")
}

fn main() {
    let mut todo_list = core::TodoList::new();
    shell::head();
    display_todos(&todo_list);
    loop {
        shell::display_options(&vec!["insert", "remove", "toggle", "list", "exit"]);
        println!("\nEnter your choice:");
        match shell::input().parse::<u8>() {
            Ok(0) => {
                println!("Enter the title of the todo:");
                todo_list.insert(shell::input());
                shell::clear_screen();
            },
            Ok(1) => {
                todo_list.remove(ask_id());
                shell::clear_screen();
            },
            Ok(2) => {
                todo_list.toggle(ask_id());
                shell::clear_screen();
            },
            Ok(3) => {
                display_todos(&todo_list);
                shell::clear_screen();
            },
            Ok(4) => {
                break;
            },
            _ => {
                println!("\nInvalid option!");
            }
        }
    }
}
