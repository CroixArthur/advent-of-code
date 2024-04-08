use std::collections::HashMap;

use crate::utility;

fn try_get_alpha(line: &str) -> Option<char> {
    let numbers = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    for el in numbers {
        if line.starts_with(el.0) {
            return Some(el.1);
        }
    }
    None
}

fn process_line(line: &str, sum: &mut i32) -> () {
    let mut first: Option<char> = None;
    let mut last: char = '0';
    let mut alpha;
    let mut c: char;

    for i in 0..line.len() {
        alpha = try_get_alpha(&line[i..]);
        c = line.chars().nth(i).unwrap();
        if alpha.is_none() && c.is_numeric() {
            alpha = Some(c)
        }
        match alpha {
            Some(f) => {
                last = f;
                if first.is_none() {
                    first = Some(f);
                }
            }
            None => (),
        }
    }
    println!("Result {:?} {}", first, last);
    match first {
        Some(f) => {
            let parsed = *sum + format!("{}{}", f, last).parse::<i32>().unwrap();
            *sum = parsed;
        }
        None => (),
    }
}

fn read_file(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut sum: i32 = 0;

    if let Ok(lines) = utility::get_file_lines(path) {
        for line in lines.flatten() {
            println!("{}", line);
            process_line(&line, &mut sum);
        }
    }

    Ok(sum)
}

pub fn run() {
    let res = read_file("src/inputs/1-2input.txt");

    match res {
        Ok(yes) => println!("Day1-2: {yes}"),
        Err(err) => println!("Day12: {err}"),
    }
}
