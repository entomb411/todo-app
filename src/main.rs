use clap::Parser;
use flexi_logger::Logger;
use std::fs;
// use flexi_logger::{Duplicate, FileSpec, Logger};

#[derive(Parser, Debug)]
struct Args {
    // Specify the todo file path.
    #[clap(
        short,
        long,
        default_value = "todo.txt",
        help = "Path to the todo file"
    )]
    file: String,
}

fn print_menu() {
    println!("1. View todo list");
    println!("2. Add a new todo");
    println!("3. Remove a todo");
    println!("4. Toggle todo completion");
    println!("5. Exit");
}

pub struct TodoItem {
    completed: bool,
    description: String,
}

// A todo item should look like:
// [ ] Buy groceries
// or
// [x] Walk the dog
fn parse_todo_item(line: &str) -> Result<TodoItem, String> {
    let completed = line.starts_with("[x] ");
    let not_completed = line.starts_with("[ ] ");
    let description = if completed || not_completed {
        line[4..].to_string()
    } else {
        return Err(format!("Invalid todo item format: {}", line));
    };
    Ok(TodoItem {
        completed,
        description,
    })
}

fn parse_todo_list(contents: String) -> Vec<TodoItem> {
    let mut items = Vec::new();
    for line in contents.lines() {
        match parse_todo_item(line) {
            Ok(item) => items.push(item),
            Err(e) => log::warn!("{}", e),
        }
    }
    items
}

fn todo_item_to_string(item: &TodoItem) -> String {
    let status = if item.completed { "[x]" } else { "[ ]" };
    format!("{} {}", status, item.description)
}

fn todo_list_to_string(todo_list: &[TodoItem]) -> String {
    todo_list
        .iter()
        .map(todo_item_to_string)
        .collect::<Vec<String>>()
        .join("\n")
}

fn print_todo_list(todo_list: &[TodoItem]) {
    let output = todo_list_to_string(todo_list);
    println!("{output}");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger.
    Logger::try_with_env_or_str("debug")?
        // .log_to_file(FileSpec::default())         // write logs to file
        // .duplicate_to_stderr(Duplicate::Warn)     // print warnings and errors also to the console
        .format(flexi_logger::colored_detailed_format) // use detailed format
        .start()?;

    // Parse command line arguments.
    let args = Args::parse();
    if args.file.is_empty() {
        return Err("Error: The file path cannot be empty.".into());
    }
    // dbg!(&args);
    log::debug!("Using todo file: {}", args.file);
    let filepath = args.file;
    let contents = match fs::read_to_string(&filepath) {
        Ok(c) => c,
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => {
            return Err(format!("Error reading file {}: {}", filepath, e).into());
        }
    };

    let todo_list = parse_todo_list(contents);
    print_todo_list(&todo_list);

    loop {
        print_menu();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let selection: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match selection {
            1 => {
                print_todo_list(&todo_list);
            }
            2 => println!("You selected {selection}"),
            3 => println!("You selected {selection}"),
            4 => println!("You selected {selection}"),
            5 => {
                println!("Exiting...");
                break;
            }
            _ => continue,
        }
    }

    fs::write(&filepath, todo_list_to_string(&todo_list))?;
    Ok(())
}
