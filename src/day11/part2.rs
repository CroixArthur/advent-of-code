use crate::utility;

fn expand_universe(universe: &mut Vec<Vec<char>>) -> Vec<(usize, usize)> {
  let mut y = 0;
  let mut x = 0;
  let mut galaxies_paths: Vec<(usize, usize)> = vec![];
  let mut y_has_galaxy = false;

  while y < universe.len() {
    if universe[y].iter().all(|&c| c == '.') {
      universe[y] = vec!['O'; universe[y].len()];
      y += 1;
    }
    y += 1;
  }
  y = 0;
  while x < universe[0].len() {
    while y < universe.len() {
      if universe[y][x] == '#' {
        galaxies_paths.push((x, y));
        y_has_galaxy = true;
      }
      y += 1;
    }
    if !y_has_galaxy {
      y = 0;
      while y < universe.len() {
        universe[y][x] = 'O';
        y += 1;
      }
    }
    y_has_galaxy = false;
    y = 0;
    x += 1;
  }

  galaxies_paths
}

fn display_map(map: &Vec<Vec<char>>) {
  for line in map {
    println!("{:?}", line);
  }
}

fn get_path_length(universe: &Vec<Vec<char>>, &fst: &(usize, usize), &snd: &(usize, usize)) -> usize {
  let mut x = fst.0;
  let mut y = fst.1;
  let mut dx: i32 = 0;
  let mut dy: i32 = 0;

  const EXP_SIZE: usize = 1000000;

  while x != snd.0 {
    if x < snd.0 {
      x += 1;
    } else {
      x -= 1;
    }
    if x != fst.0 {
      dx += match universe[y][x] {
        'O' => EXP_SIZE as i32,
        _ => 1
      }
    }
  }
  while y != snd.1 {
    if y < snd.1 {
      y += 1;
    } else {
      y -= 1;
    }
    if y != fst.1 {
      dy += match universe[y][x] {
        'O' => EXP_SIZE as i32,
        _ => 1
      };
    }
  }

  dx as usize + dy as usize
}

fn sum_up_paths(universe: &Vec<Vec<char>>, galaxies_paths: &Vec<(usize, usize)>) -> usize {
  let mut sum = 0;

  for (i, fst) in galaxies_paths[0..galaxies_paths.len()].iter().enumerate() {
    for snd in &galaxies_paths[i+1..galaxies_paths.len()] {
      sum += get_path_length(universe, fst, snd);
    }
  }

  sum
}

pub fn run() {
  let filepath = "src/input.txt";
  let mut universe: Vec<Vec<char>> = vec![];
  let galaxies_paths: Vec<(usize, usize)>;

  match utility::get_file_lines(filepath) {
    Ok(value) => {
      for line in value {
        if let Ok(res) = line {
          universe.push(res.chars().collect());
        }
      }
    }
    Err(e) => println!("{e}")
  }
  println!("Before:");
  display_map(&universe);
  galaxies_paths = expand_universe(&mut universe);
  println!("\nAfter:");
  display_map(&universe);
  println!("Sum: {}", sum_up_paths(&universe, &galaxies_paths));
}