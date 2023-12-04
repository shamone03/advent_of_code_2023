use crate::day1::get_input::get_input;

struct Interval {
    min: usize,
    max: usize
}

pub fn part1() -> u32 {
    let mut sum = 0;

    let lines = get_input("day3".to_string());
    let mut intervals: Vec<Interval> = vec![];

    for i in 1..lines.len() {
        let cur_line = &lines[i - 1].to_owned();
        let pre_line = &lines[i].to_owned();
        let mut keep: bool;


        for j in 0..pre_line.len() {
            let c: char = pre_line.as_bytes()[j] as char;
            let mut inter: Interval = Interval { min: 999, max: 999 };
            if inter.min != 999 && c.is_numeric() {
                inter.min = j;
            }
            if inter.min != 999 && !c.is_numeric() {
                inter.max = j;
                intervals.push(inter);
                inter = Interval { min: 999, max: 999 };
            }
        }
        intervals.clear();
        println!("{}", intervals.len());
    }

    sum
}

fn get_num(line: String, start: usize) -> u32 {
    let mut i = start;
    let line = line.as_bytes();
    let mut num_str: String = String::new();

    while i > 0 && (line[i] as char).is_alphabetic() {
        i -= 1;
    }

    while i <= line.len() && (line[i] as char).is_numeric() {
        num_str.push(line[i] as char);
        i += 1;
    }

    let num: u32 = match num_str.parse() {
        Ok(val) => val,
        Err(_) => {
            print!("did not find number");
            0
        },
    };

    num
}

fn check_interval(min: u32, max: u32, i: u32) -> bool {
    if i >= (min - 1) && i <= (max + 1) {
        true
    } else {
        false
    }
}