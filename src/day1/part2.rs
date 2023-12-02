use super::get_input::get_input;

pub fn part2() -> u32 {

    let lines = get_input();
    let mut first: u32 = 99;
    let mut last: u32 = 99;

    let mut sum = 0;

    let digits_front = ["one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), "nine".to_string()];
    let digits_back = ["eno".to_string(), "owt".to_string(), "eerht".to_string(), "ruof".to_string(), "evif".to_string(), "xis".to_string(), "neves".to_string(), "thgie".to_string(), "enin".to_string()];
    
    for line in lines {
        
        let line = line.as_bytes();
        let mut frontstr: String = String::new();
        let mut backstr: String = String::new();

        for i in 0..line.len() {
            let front: char = line[i] as char;
            let back: char = line[line.len() - 1 - i] as char;

            if first == 99 {
                if front.is_numeric() {
                    first = front.to_digit(10).expect("bad digit");
                } else {
                    frontstr.push(front);
                    match digits_front.iter().position(|e| frontstr.contains(e)) {
                        Some(index) => {
                            first = (index + 1) as u32;
                            frontstr = "".to_string();
                        },
                        None => {},
                    }
                }
            }
            
            if last == 99 {
                if back.is_numeric() {
                    last = back.to_digit(10).expect("bad digit");
                } else {
                    backstr.push(back);
                    match digits_back.iter().position(|e| backstr.contains(e)) {
                        Some(index) => {
                            last = (index + 1) as u32;
                            backstr = "".to_string();
                        },
                        None => {},
                    }
                }
            }
            
            if first != 99 && last != 99 {
                break;
            }
        }
        
        sum += first * 10 + last;
        
        first = 99;
        last = 99;
    }
    
    sum
}
