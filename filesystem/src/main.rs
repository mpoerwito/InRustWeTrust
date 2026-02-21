use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

fn main() {
    let path = Path::new("data/hello.txt");
    let show = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", show, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", show, why),
        Ok(_) => print!("{} contains:\n{}", show, s),
    }
}
