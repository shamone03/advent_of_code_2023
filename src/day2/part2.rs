use crate::day1::get_input::get_input;

use super::Game;
const R: u32 = 12;
const G: u32 = 13;
const B: u32 = 14;

pub fn part2() -> u32 {

    let lines = get_input("day2".to_string());


    let mut sum = 0;

    for ele in lines {
        let id: Vec<&str> = ele.split(":").collect();
        let right: Vec<_> = id[1].split(";").collect();
        let mut max_r = 0;
        let mut max_g = 0;
        let mut max_b = 0;

        for i in right {
            let turn: Vec<_> = i.split(",").collect();

            for color in turn {
                let color: Vec<_> = color.split_whitespace().collect();
                let mut r: u32 = 0;
                let mut g: u32 = 0;
                let mut b: u32 = 0;

                if color[1] == "red" {
                    r = color[0].parse().unwrap();
                    if r > max_r { max_r = r; }
                    
                }
                if color[1] == "green" {
                    g = color[0].parse().unwrap();
                    if g > max_g { max_g = g; }
                    
                }
                if color[1] == "blue" {
                    b = color[0].parse().unwrap();
                    if b > max_b { max_b = b; }
                    
                }

            }

            
        }

        

        sum += max_r * max_g * max_b;
        println!();
    }

    sum
}