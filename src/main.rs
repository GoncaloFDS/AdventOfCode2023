extern crate core;

use std::fs;

fn main() {
    day2();
}

fn day2() {
    let input = fs::read_to_string("inputs/day2.txt").unwrap();
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