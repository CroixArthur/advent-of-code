use crate::utility;

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
  if start.1 > 0 && (map[start.1 - 1][start.0] == '7' || map[start.1 - 1][start.0] == 'F' || map[start.1 - 1][start.0] == '|') {
    return true;
  }
  false
}

fn can_go_down(map: &Vec<Vec<char>>, start: (usize, usize)) -> bool {
  if start.1 < map.len() && (map[start.1 + 1][start.0] == 'J' || map[start.1 + 1][start.0] == 'L' || map[start.1 + 1][start.0] == '|') {
    return true;
  }
  false
}

fn can_go_left(map: &Vec<Vec<char>>, start: (usize, usize)) -> bool {
  if start.0 > 0 && (map[start.1][start.0 - 1] == 'L' || map[start.1][start.0 - 1] == 'F' || map[start.1][start.0 - 1] == '-') {
    return true;
  }
  false
}

fn can_go_right(map: &Vec<Vec<char>>, start: (usize, usize)) -> bool {
  if start.0 < map[start.1].len() && (map[start.1][start.0 + 1] == 'J' || map[start.1][start.0 + 1] == '7' || map[start.1][start.0 + 1] == '-') {
    return true;
  }
  false
}

fn get_next_possible_path(map: &Vec<Vec<char>>, before: (usize, usize), actual: (usize, usize)) -> Option<(usize, usize)> {
  if (map[actual.1][actual.0] == 'S'
    || map[actual.1][actual.0] == 'J'
    || map[actual.1][actual.0] == 'L'
    || map[actual.1][actual.0] == '|')
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
    && actual.0 - 1 != before.0
    && can_go_left(map, actual)
    { return Some((actual.0 - 1, actual.1)) }

  None
}

fn find_farthest_point(map: &Vec<Vec<char>>, start: (usize, usize)) -> i32 {
  let mut before = start;
  let mut actual = start;
  let mut length = 0;

  while let Some(val) = get_next_possible_path(map, before, actual) {
    if map[val.1][val.0] == 'S' {
      break;
    }
    if before != actual {
      before = actual;
    }
    actual = val;
    length += 1;
  }

  length
}

pub fn run() {
  let filepath = "src/input.txt";
  let mut map: Vec<Vec<char>> = vec![];
  let mut length = 0;

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
  println!("{:?}", map);
  if let Some(start) = get_start_point_index(&map) {
    length = find_farthest_point(&map, start);
  }
  println!("Length: {length}, Farthest: {}", (length + 1) / 2);
}