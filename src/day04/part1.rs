use crate::utility::{self, get_numbers};


fn parse_line(line: &str) -> (Vec<i32>, Vec<i32>) {
  let lines: Vec<&str> = line.split(": ").collect::<Vec<&str>>()[1]
    .split(" | ").collect();
  let mut winners: Vec<i32> = vec![];
  let mut numbers: Vec<i32> = vec![];
  match get_numbers(lines[0]) {
    Ok(res) => winners = res,
    Err(_) => ()
  }
  match get_numbers(lines[1]) {
    Ok(res) => numbers = res,
    Err(_) => ()
  }
  (winners, numbers)
}

pub fn run() {
  let filepath = "src/inputs/4input.txt";
  let mut content: Vec<Vec<char>> = vec![];
  let lines = utility::get_file_lines(filepath);
  let mut numbers: (Vec<i32>, Vec<i32>);
  let mut sum = 0;
  let mut card_value = 0;
  
  match lines {
    Ok(value) => {
      for line in value {
        if let Ok(res) = line {
          content.push(res.chars().collect())
        }
      }
    }
    Err(e) => println!("{e}")
  }
  for line in content {
    numbers = parse_line(String::from_iter(line).as_str());
    for num in numbers.1 {
      if numbers.0.contains(&num) {
        if card_value == 0 {
          card_value = 1;
        } else {
          card_value *= 2;
        }
      }
    }
    sum += card_value;
    card_value = 0;
  }
  println!("Sum: {sum}")
}