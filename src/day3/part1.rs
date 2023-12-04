use crate::day1::get_input::get_input;

pub fn part1() -> u32 {
    let mut sum = 0;

    let lines = get_input("day3".to_string());

    for i in 1..lines.len() {
        let cur_line = &lines[i - 1].to_owned();
        let pre_line = &lines[i].to_owned();
        let mut keep: bool;


        for j in 2..pre_line.len() {
            let cur_cc: char = cur_line.as_bytes()[j - 1] as char;

            let pre_lc = pre_line.as_bytes()[j - 2] as char;
            let pre_cc = pre_line.as_bytes()[j - 1] as char;
            let pre_rc = pre_line.as_bytes()[j] as char;

            if !cur_cc.is_numeric() && cur_cc != '.' {
                let left: bool = pre_lc.is_numeric(); // left diag match
                let right: bool = pre_rc.is_numeric(); // right diag match;

                keep = pre_cc.is_numeric() || left || right;
                if keep {
                    sum += get_num(pre_line.to_string(), j - 2);
                }
            }

            if cur_cc.is_numeric() {
                
                let left: bool = !pre_lc.is_numeric() && pre_lc != '.'; // left diag match
                let right: bool = !pre_rc.is_numeric() && pre_rc != '.'; // right diag match;

                keep = !pre_cc.is_numeric() && pre_cc != '.' || left || right;
                if keep {
                    sum += get_num(cur_line.to_string(), j - 2);
                }
            }

        }
        
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