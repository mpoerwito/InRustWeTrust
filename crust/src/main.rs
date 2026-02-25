use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
// Pathbuf is like a string but for file system paths that works cross platform.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    // The code below is the equivalent of the code above, but it is more verbose and less efficient.
    // let pattern = std::env::args().nth(1).expect("No pattern provided");
    // let path = std::env::args().nth(2).expect("No path provided");
    // let args = Cli { pattern, path: std::path::PathBuf::from(path) };
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

}
