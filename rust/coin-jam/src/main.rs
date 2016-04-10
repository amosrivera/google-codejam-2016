use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use std::env;
const CHARS: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let contents = read_input(env::args().nth(1).unwrap());
    let case = contents
                .lines()
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>();

    let length = case[0];
    let amount = case[1];
    let mut bases;
    let mut binary;
    let mut coins = vec![];
    let mut count: u32 = 0;
    let mut divisors = vec![];

    for n in 8192..16383 {
        binary = binary_for(n);
        bases = get_bases(&binary);
        divisors = get_divisors(&bases);

        if divisors.iter().filter(|&&n| n == 0).collect::<Vec<&u64>>().len() == 0 {
            coins.push([
                binary,
                String::from(" "),
                divisors
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
                ].concat()
            );

            count += 1;
        }

        if count == amount {
            break;
        }
    }

    save(coins);
}

fn get_bases(binary: &String) -> Vec<u64>{
    let mut bases = vec![];

    // 2 to 1
    for b in 2..11 {
        bases.push(convert(&binary, b));
    }

    return bases;
}

fn convert(value: &String, base: u64) -> u64 {
    let mut num: u64 = 0;
    for (i, c) in value.chars().rev().enumerate() {
        for (j, cc) in CHARS.chars().enumerate() {
            if c == cc {
                num += base.pow(i as u32) * j as u64;
            }
        }
    }
    return num;
}

fn get_divisors(numbers: &Vec<u64>) -> Vec<u64> {
    let mut divisors: Vec<u64> = vec![];

    for n in numbers {
        divisors.push(get_divisor_for(n));
    }

    return divisors;
}

fn get_divisor_for(number: &u64) -> u64 {
    if number == &2 || number == &3 {
        return 0;
    } else if number % 2 == 0 {
        return 2;
    } else if number % 3 == 0 {
        return 3;
    }

    let mut i = 5;
    let mut w = 2;
    while &(i * i) <= number {
        if number % i == 0 {
            return i;
        }
        i = i + w;
        w = 6 - w;
    }

    return 0;
}

fn binary_for(number: u64) -> String {
    format!("1{:b}1", number)
}

fn save (results: Vec<String>) {
    let mut options = OpenOptions::new();
    let path = Path::new("output.txt");
    options.write(true);

    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(..) => panic!("Error creating file"),
    };

    file.write(b"Case #1:\n");
    for (i, result) in results.iter().enumerate() {
        file.write(format!("{}\n", result).as_bytes());
    }
}


fn read_input(file_name: String) -> String {
    let path = Path::new(&file_name);
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(_) => panic!("Couldn't read file"),
        Ok(s) => s,
    };

    return s;
}
