use std::fs;
use clap::Parser;
use flexi_logger::Logger;
// use flexi_logger::{Duplicate, FileSpec, Logger};

#[derive(Parser, Debug)]
struct Args {
    // Specify the todo file path.
    #[clap(short, long, default_value = "todo.txt", help = "Path to the todo file")]
    file: String,
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

    fs::write(&filepath, contents)?;

    Ok(())
}
