use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn process_line(line: &str, sum: &mut i32) -> () {
    let mut first: Option<char> = None;
    let mut last: char = '0';

    for char in line.chars() {
        if char.is_numeric() {
            last = char;
            if first.is_none() {
                first = Some(char);
            }
        }
    }
    println!("Result {:?} {}", first, last);
    match first {
        Some(f) => {
            let parsed = *sum + format!("{}{}", f, last).parse::<i32>().unwrap();
            *sum = parsed;
        },
        None => (),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut sum:i32 = 0;

    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            println!("{}", line);
            process_line(&line, &mut sum);
        }
    }

    Ok(sum)
}

fn main() {
    let res = read_file("src/1input.txt");

    match res {
        Ok(yes) => println!("Day1: {yes}"),
        Err(err) => println!("Day1: {err}"),
    }
}
