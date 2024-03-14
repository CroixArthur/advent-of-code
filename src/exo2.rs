use crate::utility;

fn process_line(line: &str) -> Option<i32> {
  let mut split: Vec<&str> = line.split("; ").collect();
  let mut color_info: Vec<&str>;
  let mut header_split;
  let header;

  header_split = split[0].split(": ");
  header = header_split.next().unwrap();
  split[0] = header_split.last().unwrap();
  for draw in split {
    let colors: Vec<&str> = draw.split(", ").collect();

    for color in colors {
      color_info = color.split_whitespace().collect();
      if (color_info[1] == "blue" && color_info[0].parse::<i32>().unwrap() > 14)
        || (color_info[1] == "green" && color_info[0].parse::<i32>().unwrap() > 13)
        || (color_info[1] == "red" && color_info[0].parse::<i32>().unwrap() > 12) {
          return None
        }
    }
  }
  match utility::get_numbers(header) {
    Ok(id) => return Some(id[0]),
    Err(_e) => return None
  }
}

pub fn run() {
  let filepath = "src/inputs/2input.txt";
  let mut sum = 0;
  
  if let Ok(lines) = utility::get_file_lines(filepath) {
    for line in lines.flatten() {
      match process_line(&line) {
        Some(id) => sum += id,
        None => ()
      }
    }
  }
  println!("Sum: {sum}");
}