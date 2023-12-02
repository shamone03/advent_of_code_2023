use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::env;

pub fn get_input() -> Vec<String> {
    let path = Path::new("input/day1/input.txt");
    let curdir = env::current_dir().expect("cannot find cur dir");
    let path = curdir.join(path);

    println!("{}", path.display());
    
    let mut handle = match File::open(curdir.join(path)) {
        Ok(file) => file,
        Err(err) => panic!("couldn't open file: {err}"),
    };

    let mut content = String::new();

    match handle.read_to_string(&mut content) {
        Ok(_) => println!("read from file"),
        Err(err) => panic!("couldn't read file: {err}"),
    }

    return Vec::from_iter(content.split("\n").map(String::from));
}