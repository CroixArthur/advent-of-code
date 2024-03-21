use std::{fs::File, io::{self, BufRead}, path::Path};

pub fn get_file_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_file_array(filepath: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
  let mut content: Vec<Vec<char>> = vec![];
  let lines = get_file_lines(filepath);
  
  match lines {
    Ok(value) => {
      for line in value {
        if let Ok(res) = line {
          content.push(res.chars().collect())
        }
      }
    }
    Err(e) => return Err(e)
  }
  Ok(content)
}

pub fn get_numbers(str: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
  let mut numbers: Vec<i32> = vec![];
  let mut number = String::new();

  for c in str.chars() {
    if c.is_digit(10) {
      number.push(c);
    } else if !number.is_empty() {
      match number.parse::<i32>() {
        Ok(value) => { numbers.push(value) }
        Err(e) => return Err(e)
      }
      number.clear();
    }
  }
  if !number.is_empty() {
    match number.parse::<i32>() {
      Ok(value) => { numbers.push(value) }
      Err(e) => return Err(e)
    }
  }
  return Ok(numbers);
}