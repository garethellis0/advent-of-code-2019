use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::num::ParseIntError;
use std::thread::current;

fn part1(f: &File) -> Result<i64, ParseIntError> {
    let buffered = BufReader::new(f);

    let mut total_fuel: i64 = 0;

    for line in buffered.lines() {
        match line.unwrap().parse::<i64>() {
            Err(e) => return Err(e),
            Ok(i) => total_fuel += ((i as f64 / 3.0).floor() as i64) - 2,
        }
    }

    return Ok(total_fuel);
}

fn part2(f: &File) -> Result<i64, ParseIntError> {
    let buffered = BufReader::new(f);

    let mut total_fuel: i64 = 0;

    for line in buffered.lines() {
        match line.unwrap().parse::<i64>() {
            Err(e) => return Err(e),
            Ok(mut i) => {
                let mut current_fuel = i;
                while (current_fuel > 0) {
                    current_fuel = ((current_fuel as f64 / 3.0).floor() as i64) - 2;
                    if (current_fuel > 0){
                        total_fuel += current_fuel;
                    }
                }
            },
        }
    }

    return Ok(total_fuel);
}

fn main() -> Result<(), Error> {
    // Attempt to read the file
    let filepath = "part_1.dat";
    let file = File::open(filepath);
    match file {
        Err(e) => return Err(e),
        Ok(v) => {
//            match part1(&v) {
//                Err(e) => println!("Int parsing failed"),
//                Ok(i) => println!("We need {} bits of fuel!", i)
//            }
            match part2(&v){
                Err(e) => println!("Int parsing failed"),
                Ok(i) => println!(" But oh wait we actually need {} bits....... I think......", i)
            }
        }
    }

    return Ok(());
}
