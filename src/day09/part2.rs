use crate::utility;

fn get_next_line(line: &Vec<i32>) -> Vec<i32> {
  let mut vector: Vec<i32> = vec![];
  let mut it = line.iter().peekable();

  while it.peek().is_some() {
    if let Some(val1) = it.next() {
      if let Some(&val2) = it.peek() {
        vector.push(val2 - val1);
      }
    }
  }

  vector
}

fn find_next_value(line: &str) -> i32 {
  let mut map: Vec<Vec<i32>> = vec![];
  let mut index = 0;

  map.push(line.split(' ').map(|el| el.parse::<i32>().unwrap()).collect());

  while map[index]
    .iter()
    .filter(|&val| *val == 0)
    .collect::<Vec<&i32>>()
    .len() != map[index].len() {
      index += 1;
      map.push(get_next_line(&map[index - 1]));
  }
  map[index].push(0);
  while index != 0 {
    let last_line = *map[index].first().unwrap();
    index -= 1;
    let current_line = &mut map[index];
    current_line.insert(0, current_line.first().unwrap() - last_line)
  }

  *map[0].first().unwrap()
}

pub fn run() {
  let filepath = "src/inputs/9input.txt";
  let mut sum = 0;

  match utility::get_file_lines(filepath) {
    Ok(value) => {
      for line in value {
        if let Ok(res) = line {
          sum += find_next_value(&res);
        }
      }
    }
    Err(e) => println!("{e}")
  }
  println!("Sum: {sum}");
}