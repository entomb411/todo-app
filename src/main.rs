use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    // Specify the todo file path.
    #[clap(short, long, default_value = "todo.txt", help = "Path to the todo file")]
    file: String,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.file.is_empty() {
        return Err("Error: The file path cannot be empty.".into());
    }
    dbg!(&args);
    let filepath = args.file;
    let contents = match fs::read_to_string(&filepath) {
        Ok(c) => c,
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => {
            return Err(format!("Error reading file {}: {}", filepath, e).into());
        }
    };

    fs::write(&filepath, contents)?;

    Ok(())
}
