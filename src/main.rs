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
    println!("4. Exit");
}

fn print_todo_list(contents: &str) {
    if contents.trim().is_empty() {
        println!("Your todo list is empty.");
    } else {
        // println!("Your todo list:");
        for (index, line) in contents.lines().enumerate() {
            println!("{}. {}", index + 1, line);
        }
    }
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

    print_todo_list(&contents);

    loop {
        print_menu();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let selection: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match selection {
            1 => println!("You selected 1"),
            2 => println!("You selected 2"),
            3 => println!("You selected 3"),
            4 => {
                println!("Exiting...");
                break;
            }
            _ => continue,
        }
    }

    fs::write(&filepath, &contents)?;
    Ok(())
}
