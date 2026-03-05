// main.rs — A CLI app example using `clap` for argument parsing
//
// Example usage:
//   cargo run -- --name Alice --age 30 --role admin --verbose

// The `derive` feature lets us use #[derive(Parser)] on a struct
use clap::Parser;

// ---------------------------------------------------------------------------
// 1. DEFINE YOUR ARGUMENTS AS A STRUCT
//    Each field becomes a CLI argument. Clap reads the field names, types,
//    and doc-comments to auto-generate --help text.
// ---------------------------------------------------------------------------
#[derive(Parser, Debug)]
#[command(
    name = "user-tool",          // Binary name shown in help
    version = "1.0.0",           // Shown with --version
    about = "A demo CLI tool that accepts and validates user info",
    long_about = None,
)]
struct Args {
    /// Full name of the user (required, must be at least 2 characters)
    #[arg(short, long)]          // Generates both -n and --name
    name: String,

    /// Age of the user (required, must be 1–120)
    #[arg(short, long)]
    age: u8,                     // u8 means clap rejects non-numeric input automatically

    /// Role of the user: admin, editor, or viewer (default: viewer)
    #[arg(short, long, default_value = "viewer")]
    role: String,

    /// Optional tag to attach to the user (can be supplied multiple times)
    /// e.g. --tag rust --tag cli
    #[arg(short, long)]
    tag: Vec<String>,            // Vec<T> lets users pass the flag multiple times

    /// Enable verbose output
    #[arg(short, long, default_value_t = false)]
    verbose: bool,               // bool flags are on/off switches (--verbose or omit)
}

// ---------------------------------------------------------------------------
// 2. VALIDATION LOGIC
//    Clap handles type checking; we add business-rule validation here.
// ---------------------------------------------------------------------------
fn validate_args(args: &Args) -> Result<(), String> {
    // Rule: name must be at least 2 characters
    if args.name.trim().len() < 2 {
        return Err(format!(
            "Name '{}' is too short — must be at least 2 characters.",
            args.name
        ));
    }

    // Rule: age must be realistic (clap enforces u8 so it's already 0–255)
    if args.age == 0 || args.age > 120 {
        return Err(format!(
            "Age {} is out of the valid range (1–120).",
            args.age
        ));
    }

    // Rule: role must be one of the allowed values
    let allowed_roles = ["admin", "editor", "viewer"];
    if !allowed_roles.contains(&args.role.as_str()) {
        return Err(format!(
            "Role '{}' is not valid. Choose from: {}.",
            args.role,
            allowed_roles.join(", ")
        ));
    }

    Ok(()) // All checks passed
}

// ---------------------------------------------------------------------------
// 3. DISPLAY THE PARSED ARGUMENTS
// ---------------------------------------------------------------------------
fn display_args(args: &Args) {
    println!("\n=== Parsed Arguments ===");
    println!("  Name    : {}", args.name);
    println!("  Age     : {}", args.age);
    println!("  Role    : {}", args.role);

    // Vec<String> — display as comma-separated list, or "(none)" if empty
    if args.tag.is_empty() {
        println!("  Tags    : (none)");
    } else {
        println!("  Tags    : {}", args.tag.join(", "));
    }

    println!("  Verbose : {}", args.verbose);

    // Only print extra details if --verbose was passed
    if args.verbose {
        println!("\n[verbose] Name length  : {} chars", args.name.len());
        println!("[verbose] Tag count    : {}", args.tag.len());
        println!(
            "[verbose] Permissions : {}",
            if args.role == "admin" { "read, write, delete" } else { "read only" }
        );
    }
}

// ---------------------------------------------------------------------------
// 4. ENTRY POINT
// ---------------------------------------------------------------------------
fn main() {
    // `parse()` reads std::env::args(), matches against the struct definition,
    // and exits with a helpful error message if anything is wrong.
    let args = Args::parse();

    // Run our custom validation on top of clap's type-level checks
    if let Err(e) = validate_args(&args) {
        // eprintln! writes to stderr — correct for error messages
        eprintln!("Error: {e}");
        std::process::exit(1); // Non-zero exit code signals failure to the shell
    }

    // All good — display the results
    display_args(&args);
}
