use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
extern crate walkdir;
use walkdir::WalkDir;

fn main() {
    // let path = Path::new("data/hello.txt");
    // let show = path.display();

    // let mut file = match File::open(&path) {
    //     Err(why) => panic!("couldn't open {}: {}", show, why),
    //     Ok(file) => file,
    // };

    // let mut s = String::new();
    // match file.read_to_string(&mut s) {
    //     Err(why) => panic!("couldn't read {}: {}", show, why),
    //     Ok(_) => print!("{} contains:\n{}", show, s),
    //}

    // read_file().map(|s| println!("{}", s)).unwrap_or_else(|err| {
    //     println!("Error reading file: {}", err);
    // });
    get_files();

}

fn get_files() {
    let dirpath = Path::new("../networking");
    for entry in WalkDir::new(dirpath).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && entry.file_name().to_str().unwrap().starts_with(".") {
            println!("{}", entry.path().display());
        }
    }
    // Count all files and folders in directory:
    println!("{}", fs::read_dir(dirpath).unwrap().count());
    // Recursively count all files and folders in directory and subdirectories:
    println!("{}", WalkDir::new(dirpath).into_iter().count());
    // let paths = std::fs::read_dir(dirpath)?;

    // for path in paths {
    //     println!("Name: {}", path?.path().display())
    // }
    // Ok(())
}


fn read_file() -> Result<String, std::io::Error> {
    let path = Path::new("data/hello.txt");
    let mut file = File::open(&path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}