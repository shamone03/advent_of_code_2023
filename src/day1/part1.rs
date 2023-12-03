use super::get_input::get_input;

pub fn part1() -> u32 {

    let lines = get_input("day1".to_string());
    let mut first: char = ' ';
    let mut last: char = ' ';

    let mut sum = 0;


    for line in lines {

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

            if first != ' ' && last != ' ' {
                break;
            }

        }
        sum += first.to_digit(10).expect("bad digit") * 10 + last.to_digit(10).expect("bad digit");

        first = ' ';
        last = ' ';
    }
    
    sum
}
