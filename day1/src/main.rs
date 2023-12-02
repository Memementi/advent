use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a filename");
        return;
    }
    let filename = &args[1];

    println!("In file: {}", filename);

    if let Ok(lines) = read_lines(filename) {
        let mut calibration_total = 0;
        for line in lines {
            if let Ok(text) = line {
                calibration_total += get_calibration_value(text);
            }
        }
        println!("Total: {}", calibration_total);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_calibration_value(line: String) -> u32 {
    let mut first_int = 99;
    let mut last_int = 99;
    for character in line.chars() {
        if character.is_digit(10) {
            let digit = character.to_digit(10).unwrap();
            last_int = digit;
            if first_int == 99 {
                first_int = digit;
            }
        }
    }
    first_int*10 + last_int
}