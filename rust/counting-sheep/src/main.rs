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
            get_last_digit(
                case.parse().unwrap()
            )
        )
    }

    save(results);
}

fn get_last_digit(number: u32) -> String {
    const INFINITE: &'static str = "INSOMNIA";
    let mut count = 0;
    let mut result;
    let mut numbers = Vec::new();

    if number == 0 {
        return String::from(INFINITE);
    }

    loop {
        count += 1;
        result = (number * count).to_string();

        numbers.extend(
            result
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
                .iter()
                .cloned()
        );

        numbers.sort();
        numbers.dedup();

        if numbers.len() == 10 {
            break;
        }
    }

    return result;
}

fn save (results: Vec<String>) {
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
