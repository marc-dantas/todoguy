extern crate rusqlite;
extern crate console;
mod core;
use crate::core::*;
use console::*;
use std::io::{stdout, Write};

fn show_help() {
    Term::stdout().clear_screen().unwrap();
    println!("{}", style("HELP").bold().cyan());
    println!("press [{}] to open help message.", style("H").bold().red());
    println!("press [{}] to open credits.\n", style("C").bold().red());
    println!("use [{}] and [{}] to move up and down.", style("W").bold().red(), style("S").bold().red());
    println!("use [{}] or [{}] to toggle the check mark.", style("A").bold().red(), style("D").bold().red());
    println!("press [{}] to create a new todo.", style("N").bold().red());
    println!("press [{}] to delete a todo.", style("X").bold().red());
    println!("press [{}] to quit.\n", style("Q").bold().red());
    println!("{}", style("press any key to continue").italic());
    getch().unwrap();
}

fn show_credits() {
    Term::stdout().clear_screen().unwrap();
    println!("{}", style("CREDITS").bold().cyan());
    println!("welcome to {}, a terminal-based todo-list application.", style("TodoGuy").bold().cyan());
    println!("this is {} software licensed under the {}.", style("open source").bold(), style("MIT License").bold());
    println!("by {}.\n", style("Marcio Dantas").bold().cyan());
    println!("TodoGuy on GitHub: {}", style("https://github.com/marc-dantas/todoguy/").underlined().blue());
    println!("Marcio Dantas on Github: {}\n", style("https://github.com/marc-dantas/").underlined().blue());
    println!("{}", style("press any key to continue").italic());
    getch().unwrap();
}

fn main() {
    let term = Term::stdout();
    let db = setup_database("data.db");
    let mut todos = Todo::load(&db);
    let mut selection = 0;
    loop {
        let len = todos.len();
        term.clear_screen().unwrap();
        println!("{}", style("TodoGuy").bold());
        Todo::update_list(&db, &mut todos);
        Todo::list(&db, selection);
        if let Some(c) = getch() {
            match c {
                'q' => {
                    print!("{} (y/N) ", style("are you sure?").bold().red());
                    stdout().flush().unwrap();
                    if let Some(c) = getch() {
                        match c {
                            'y'|'Y' => break,
                            _ => {}
                        }
                    }
                },
                'n' => {
                    print!("{}", style("new todo: ").bold().cyan());
                    stdout().flush().unwrap();
                    if let Some(value) = getline() {
                        Todo::new(
                            &db,
                            value,
                            false
                        ).unwrap();
                    }
                },
                'h' => { show_help(); continue; },
                'c' => { show_credits(); continue; },
                _ if todos.is_empty() => {},
                'w' if selection > 0 => selection -= 1,
                's' if (selection+1) < len => selection += 1,
                'a'|'d' => {
                    Todo::toggle(&db, &todos, selection)
                },
                'x' => {
                    print!("{} (y/N) ", style("are you sure?").bold().red());
                    stdout().flush().unwrap();
                    if let Some(c) = getch() {
                        match c {
                            'y'|'Y' => Todo::delete(&db, &todos, selection),
                            _ => {}
                        }
                    }
                    selection -= if selection == 0 { 0 } else { 1 };
                },
                _ => {},
            }
        } else {
            println!("{}", style("invalid input").red());
            continue;
        }
    }
}
