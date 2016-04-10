use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let contents = read_input("input.txt");
    let mut lines: Vec<&str> = contents.lines().collect();
    let mut results = Vec::new();
    lines.remove(0); // number of cases is not used

    for case in lines {
        results.push(
            solve(
                case
                    .chars()
                    .map(|p| p == '+')
                    .collect()
            )
        )
    }

    save(results);
}

fn solve(mut pancakes: Vec<bool>) -> u32 {
    let mut flips = 0;
    let mut first;

    loop {
        first = pancakes[0];

        match pancakes.iter().position(|&p| p != first) {
            Some(index) => {
                pancakes = flip_from(pancakes, index);
            },
            None => {
                if first == false {
                    flips += 1;
                }
                break;
            },
        }

        flips += 1;
    }

    return flips;
}

fn flip_from(mut pancakes: Vec<bool>, index: usize) -> Vec<bool> {
    let mut chunk: Vec<bool> = pancakes.split_off(index);

    pancakes = pancakes.iter().map(|p| !p).collect::<Vec<bool>>();
    pancakes.extend(chunk);

    return pancakes;
}

fn save (results: Vec<u32>) {
    let mut options = OpenOptions::new();
    let path = Path::new("output.txt");
    options.write(true);

    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(..) => panic!("Error creating file"),
    };

    for (i, result) in results.iter().enumerate() {
        file.write(
            format!("Case #{}: {}\n", i + 1, result).as_bytes()
        );
    }
}

fn read_input(file_name: &'static str) -> String {
    let path = Path::new(file_name);
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(_) => panic!("Couldn't read file"),
        Ok(s) => s,
    };

    return s;
}
