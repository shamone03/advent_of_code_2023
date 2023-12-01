use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::env;
fn main() {

    let path = Path::new("input/inputs.txt");
    let curdir = env::current_dir().expect("cannot find cur dir");
    let path = curdir.join(path);

    
    let mut handle = match File::open(curdir.join(path)) {
        Ok(file) => file,
        Err(err) => panic!("couldn't open file: {err}"),
    };

    let mut content = String::new();

    match handle.read_to_string(&mut content) {
        Ok(_) => println!("{content}"),
        Err(err) => panic!("couldn't read file: {err}"),
    }

    

    println!("Hello, world!");
}
