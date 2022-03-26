// Author: @marc-dantas
// <https://github.com/marc-dantas>
mod shell;
mod core;

fn display_todos(todo_list: &mut core::TodoList) {
    println!("------ Todos ------");
    todo_list.list();
    println!("-------------------");
}

fn handle_input_values(value: &u8, todo_list: &mut core::TodoList) {
    match value {
        0 => {
            let title = shell::prompt("Enter the title of the todo:");
            todo_list.insert(title);
            shell::clear_screen();
        }
        1 => {
            todo_list.remove(shell::int_prompt("Enter the id of the todo you want to delete:"));
            shell::clear_screen();
        }
        2 => {
            todo_list.toggle(shell::int_prompt("Enter the id of the todo you want to toggle:"));
            shell::clear_screen();
        }
        3 => { // exit
            std::process::exit(0);
        }
        _ => {
            println!("Invalid option");
        }
    }
}

fn handle_input_commands<T>(input: &Result<u8, T>,
                        todo_list: &mut core::TodoList) {
    match input {
        Ok(value) => handle_input_values(value, todo_list),
        Err(_) => println!("Error"),
    }
}


fn main() {
    let mut todo_list = core::TodoList::new();
    shell::head();
    display_todos(&mut todo_list);
    loop {
        shell::display_options(&vec![
            "insert", "remove", "toggle", "exit"
        ]);
        let input = shell::prompt("Enter your choice:")
                            .trim().parse::<u8>();
        handle_input_commands(&input, &mut todo_list);
        display_todos(&mut todo_list);
    }
}
