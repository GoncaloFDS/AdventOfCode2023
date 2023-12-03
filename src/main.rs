extern crate core;

use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day3.txt").unwrap();
    //day2(input);
    day3(input);
}

fn day2(input: String) {
    let sum: i32 = input.lines().enumerate().map(|(id, line)| {
        let mut split = line.split(":").skip(1);
        let results = split.next().unwrap();
        let mut rounds = results.split(";");
        let mut mins = [0, 0, 0];
        rounds.for_each(|round| {
            round.split(",").for_each(|set| {
                let mut set = set.split(" ").skip(1);
                let left = set.next().unwrap();
                let right = set.next().unwrap();
                match (left.parse::<i32>().unwrap(), right) {
                    (number, "red") => {
                        if number > mins[0] { mins[0] = number }
                    }
                    (number, "green") => {
                        if number > mins[1] { mins[1] = number }
                    }
                    (number, "blue") => {
                        if number > mins[2] { mins[2] = number }
                    }
                    (_) => panic!("failed to parse")
                };
            });
        });
        println!("{mins:?}");
        mins.iter().fold(1i32, |acc, x| acc * x)
    }).sum();
    println!("sum: {sum}");
}

fn day3(input: String) {
    let engine: Vec<Vec<char>> =
        input.lines().map(|line| {
            line.chars().map(|char| char).collect()
        }).collect();


    let mut numbers = vec![];
    for y in 0..engine.len()
    {
        let mut current_number = String::new();
        let mut start = None;
        for x in 0..engine[y].len() {
            let char = engine[y][x];
            if char.is_numeric() {
                current_number += char.to_string().as_ref();
                if start.is_none() {
                    start = Some((x, y))
                }
            } else {
                if !current_number.is_empty()
                {
                    numbers.push((current_number, start));
                }
                current_number = String::new();
                start = None
            }
        }
    }
    println!("{numbers:?}");

    let sum: i32 = numbers.iter().map(|(s, pos)| {
        let (sx, sy) = pos.unwrap();
        let sx = sx as i32;
        let sy = sy as i32;

        let mut valid = false;
        for y in (sy - 1)..(sy + 2) {
            for x in (sx - 1)..(sx + s.len() as i32 + 1) {
                if y < 0 || x < 0 { print!("(!)"); continue; };
                if let Some(a) = engine.get(y as usize) {
                    if let Some(char) = a.get(x as usize) {
                        if !char.is_numeric() && !char.eq(&'.') {
                            print!("[{char}]");
                            valid = true;
                        }
                        else{
                            print!("({char})");

                        }
                    }
                }
            }
            print!("\n")
        }

        print!("\n");
        if valid {
            //println!("found");
            let a = s.parse::<i32>().unwrap();
            println!("{}", a);
            a
        } else {
            //println!("not found");
            0
        }
    }).sum();
    println!("sum {sum}");
}