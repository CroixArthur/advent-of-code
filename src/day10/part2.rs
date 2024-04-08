use crate::utility;

// fn print_bool(cond: bool) -> String {
//   if cond {
//     return "\x1b[0;32mtrue\x1b[0m".to_owned();
//   } else {
//     return "\x1b[0;31mfalse\x1b[0m".to_owned();
//   }
// }

fn get_x_occurences(map: &Vec<Vec<char>>, shape: &Vec<(usize, usize)>, start: &(usize, usize)) -> usize {
  let mut x: usize = start.0;
  let mut crossings = 0;
  let mut up_crossed = false;
  let mut down_crossed = false;
  
  while let Some(val) = map[start.1].get(x) {
    if shape.contains(&(x, start.1)) {
      match val {
        'J' => {
          if down_crossed {
            crossings += 1;
          }
          up_crossed = false;
          down_crossed = false;
        },
        '7' => {
          if up_crossed {
            crossings += 1;
          }
          up_crossed = false;
          down_crossed = false;
        },
        'L' => up_crossed = true,
        'F' => down_crossed = true,
        '|' => crossings += 1,
        'S' => {
          up_crossed = false;
          down_crossed = false;
        },
        _ => ()
      }
    }
    x += 1;
  }

  crossings
}

fn is_point_inside(map: &Vec<Vec<char>>, shape: &Vec<(usize, usize)>, point: (usize, usize)) -> bool {
  let x_occurences = get_x_occurences(map, shape, &point);
  
  x_occurences != 0 && x_occurences % 2 == 1
}

fn get_start_point_index(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
  for (i, line) in map.iter().enumerate() {
    for (j, c) in line.iter().enumerate() {
      if *c == 'S' {
        return Some((j, i));
      }
    }
  }
  None
}

fn can_go_up(map: &Vec<Vec<char>>, start: (usize, usize)) -> bool {
  if start.1 > 0 && (map[start.1 - 1][start.0] == '7'
    || map[start.1 - 1][start.0] == 'F'
    || map[start.1 - 1][start.0] == '|'
    || map[start.1 - 1][start.0] == 'S') {
    return true;
  }
  false
}

fn can_go_down(map: &Vec<Vec<char>>, start: (usize, usize)) -> bool {
  if start.1 < map.len() && (map[start.1 + 1][start.0] == 'J'
    || map[start.1 + 1][start.0] == 'L'
    || map[start.1 + 1][start.0] == '|'
    || map[start.1 + 1][start.0] == 'S') {
    return true;
  }
  false
}

fn can_go_left(map: &Vec<Vec<char>>, start: (usize, usize)) -> bool {
  if start.0 > 0 && (map[start.1][start.0 - 1] == 'L'
    || map[start.1][start.0 - 1] == 'F'
    || map[start.1][start.0 - 1] == '-'
    || map[start.1][start.0 - 1] == 'S') {
    return true;
  }
  false
}

fn can_go_right(map: &Vec<Vec<char>>, start: (usize, usize)) -> bool {
  if start.0 < map[start.1].len() && (map[start.1][start.0 + 1] == 'J'
    || map[start.1][start.0 + 1] == '7'
    || map[start.1][start.0 + 1] == '-'
    || map[start.1][start.0 + 1] == 'S') {
    return true;
  }
  false
}

fn get_next_possible_path(map: &Vec<Vec<char>>, before: (usize, usize), actual: (usize, usize)) -> Option<(usize, usize)> {
  if (map[actual.1][actual.0] == 'S'
    || map[actual.1][actual.0] == 'J'
    || map[actual.1][actual.0] == 'L'
    || map[actual.1][actual.0] == '|')
    && actual.1 > 0
    && actual.1 - 1 != before.1
    && can_go_up(map, actual)
    { return Some((actual.0, actual.1 - 1)) }
  if (map[actual.1][actual.0] == 'S'
    || map[actual.1][actual.0] == 'L'
    || map[actual.1][actual.0] == 'F'
    || map[actual.1][actual.0] == '-')
    && actual.0 + 1 != before.0
    && can_go_right(map, actual)
    { return Some((actual.0 + 1, actual.1)) }
  if (map[actual.1][actual.0] == 'S'
    || map[actual.1][actual.0] == '7'
    || map[actual.1][actual.0] == 'F'
    || map[actual.1][actual.0] == '|')
    && actual.1 + 1 != before.1
    && can_go_down(map, actual)
    { return Some((actual.0, actual.1 + 1)) }
  if (map[actual.1][actual.0] == 'S'
    || map[actual.1][actual.0] == 'J'
    || map[actual.1][actual.0] == '7'
    || map[actual.1][actual.0] == '-')
    && actual.0 > 0
    && actual.0 - 1 != before.0
    && can_go_left(map, actual)
    { return Some((actual.0 - 1, actual.1)) }

  None
}

fn find_enclosed_tiles_number(map: &mut Vec<Vec<char>>, start: (usize, usize)) -> i32 {
  let mut before = start;
  let mut actual = start;
  let mut number = 0;
  let mut shape: Vec<(usize, usize)> = vec![];

  while let Some(val) = get_next_possible_path(map, before, actual) {
    shape.push(val);
    if map[val.1][val.0] == 'S' {
      break;
    }
    if before != actual {
      before = actual;
    }
    actual = val;
  }
  for i in 0..map.len() {
    for j in 0..map[i].len() {
      match !shape.contains(&(j, i)) && is_point_inside(map, &shape, (j, i)) {
        true => {
          number += 1;
          map[i][j] = 'I'
        },
        false => ()
      }
    }
  }

  print_map(&map, &shape);

  number
}

fn print_map(map: &Vec<Vec<char>>, shape: &Vec<(usize, usize)>) {
  for (y, line) in map.iter().enumerate() {
    for (x, &c) in line.iter().enumerate() {
      if c == 'I' {
        print!("\x1b[0;33mI\x1b[0m");
      } else if shape.contains(&(x, y)) {
        print!("\x1b[0;32m{c}\x1b[0m");
      } else {
        print!("{c}");
      }
    }
    print!("\n");
  }
}

pub fn run() {
  let filepath = "src/input.txt";
  let mut map: Vec<Vec<char>> = vec![];
  let mut number = 0;

  match utility::get_file_lines(filepath) {
    Ok(value) => {
      for line in value {
        if let Ok(res) = line {
          map.push(res.chars().collect());
        }
      }
    }
    Err(e) => println!("{e}")
  }
  if let Some(start) = get_start_point_index(&map) {
    number = find_enclosed_tiles_number(&mut map, start);
  }
  println!("Number: {number}");
}