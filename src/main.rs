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
        Ok(_) => println!("read from file"),
        Err(err) => panic!("couldn't read file: {err}"),
    }

    let lines = content.split("\n");
    let mut first: char = ' ';
    let mut last: char = ' ';
    let test = 'd';
    println!("{test}");

    for line in lines {
        println!("{line}");
        let line = line.as_bytes();
        for i in 0..line.len() {
            let front: char = line[i] as char;
            let back: char = line[line.len() - 1 - i] as char;

            if first == ' ' {
                if front.is_numeric() {
                    first = front;
                }
            }

            if last == ' ' {
                if back.is_numeric() {
                    last = back;
                }
            }


        }

        println!("{first}{last}");

    }

    println!("Hello, world!");
}
