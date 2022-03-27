use std::io::stdin;

const VERSION: &str = "0.1";

pub fn head() {
    let mut title: String = "\nTodoGuy\nV.".to_string();
    title.push_str(VERSION);
    title.push_str("\nby @marc-dantas");
    println!("{}\n", title);
}

pub fn input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_lowercase().to_string()
}

pub fn prompt(message: &str) -> String {
    println!("{}", message);
    input()
}

pub fn int_prompt(message: &str) -> i32 {
    match prompt(message).trim().parse::<i32>() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid option");
            0
        },
    }
}


pub fn display_options(opts: &Vec<&str>) {
    println!("\nOptions:");
    for (i, opt) in opts.iter().enumerate() {
        println!("\t{}: {}", i, opt);
    }
}

pub fn clear_screen() {
    print!("{}[2J", 27 as char);
}