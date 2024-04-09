use crate::utility;

fn expand_universe(universe: &mut Vec<Vec<char>>) -> Vec<(usize, usize)> {
  let mut y = 0;
  let mut x = 0;
  let mut galaxies_paths: Vec<(usize, usize)> = vec![];
  let mut y_has_galaxy = false;

  while y < universe.len() {
    if universe[y].iter().all(|&c| c == '.') {
      universe.insert(y, universe[y].to_vec());
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
        universe[y].insert(x, '.');
        y += 1;
      }
      x += 1;
    }
    y_has_galaxy = false;
    y = 0;
    x += 1;
  }

  galaxies_paths
}

fn sum_up_paths(galaxies_paths: &Vec<(usize, usize)>) -> usize {
  let mut sum = 0;

  for (i, fst) in galaxies_paths[0..galaxies_paths.len()].iter().enumerate() {
    for snd in &galaxies_paths[i+1..galaxies_paths.len()] {
      let tmp = (snd.0 as i32 - fst.0 as i32).abs() + (snd.1 as i32 - fst.1 as i32).abs();
      sum += tmp as usize;
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
  galaxies_paths = expand_universe(&mut universe);
  println!("Sum: {}", sum_up_paths(&galaxies_paths))
}