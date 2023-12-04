use crate::day1::get_input::get_input;

pub fn part1() -> u32 {
    let sum = 0;

    let lines = get_input("day3".to_string());

    for i in 1..lines.len() {
        let cur_line = &lines[i - 1].to_owned();
        let pre_line = &lines[i].to_owned();

        for c in cur_line.chars() {
            
        }
        
    }

    sum
}