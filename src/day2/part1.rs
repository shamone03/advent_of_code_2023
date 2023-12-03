use crate::day1::get_input::get_input;

use super::Game;
const R: u32 = 12;
const G: u32 = 13;
const B: u32 = 14;

pub fn part1() -> u32 {

    let lines = get_input("day2".to_string());
    let games: Vec<Game>;

    let mut sum = 0;

    for ele in lines {
        let id: Vec<&str> = ele.split(":").collect();
        let gameid: Vec<_> = id[0].split_whitespace().collect();
        let right: Vec<_> = id[1].split(";").collect();

        let idnum: u32 = gameid[1].parse().unwrap();
        
        let mut valid = true;
        for i in right {
            let turn: Vec<_> = i.split(",").collect();

            for color in turn {
                let color: Vec<_> = color.split_whitespace().collect();
                let mut r: u32 = 0;
                let mut g: u32 = 0;
                let mut b: u32 = 0;

                if color[1] == "red" {
                    r = color[0].parse().unwrap();
                    if r > R {
                        valid = false;
                        break;
                    }
                    print!("r: {r}");
                }
                if color[1] == "green" {
                    g = color[0].parse().unwrap();
                    if g > G {
                        valid = false;
                        break;
                    }

                    print!("g: {g}");
                }
                if color[1] == "blue" {
                    b = color[0].parse().unwrap();
                    if b > B {
                        valid = false;
                        break;
                    }
                    print!("b: {b}");
                }

            }

            
        }
        if valid {
            sum += idnum;
        }
        println!();
    }

    sum
}