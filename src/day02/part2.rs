use crate::utility;

struct Colors {
  blue: i32,
  green: i32,
  red: i32
}

fn process_line_part_one(line: &str) -> Option<i32> {
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

fn process_line_part_two(line: &str) -> i32 {
  let mut split: Vec<&str> = line.split("; ").collect();
  let mut color_info: Vec<&str>;
  let mut max_colors = Colors {
    blue: 0,
    green: 0,
    red: 0
  };

  split[0] = split[0].split(": ").last().unwrap();
  for draw in split {
    let colors: Vec<&str> = draw.split(", ").collect();

    for color in colors {
      color_info = color.split_whitespace().collect();
      let nb = color_info[0].parse::<i32>().unwrap();
      if color_info[1] == "blue" && nb > max_colors.blue {
        max_colors.blue = nb;
      }
      if color_info[1] == "green" && nb > max_colors.green {
        max_colors.green = nb;
      }
      if color_info[1] == "red" && nb > max_colors.red {
        max_colors.red = nb;
      }
    }
  }
  return max_colors.blue * max_colors.green * max_colors.red;
}

pub fn run_part_one() {
  let filepath = "src/inputs/2input.txt";
  let mut sum = 0;
  
  if let Ok(lines) = utility::get_file_lines(filepath) {
    for line in lines.flatten() {
      match process_line_part_one(&line) {
        Some(id) => sum += id,
        None => ()
      }
    }
  }
  println!("Sum: {sum}");
}

pub fn run_part_two() {
  let filepath = "src/inputs/2input.txt";
  let mut sum = 0;
  
  if let Ok(lines) = utility::get_file_lines(filepath) {
    for line in lines.flatten() {
      sum += process_line_part_two(&line);
    }
  }
  println!("Sum: {sum}");
}