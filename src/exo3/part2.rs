use crate::utility;

fn retreive_number(row: &mut Vec<char>, j: usize) -> i32 {
  let mut num = String::from("");
  let mut tj = j;

  while tj > 0 && row[tj].is_numeric() {
    tj -= 1;
  }
  if !row[tj].is_numeric() {
    tj += 1;
  }
  while tj < row.len() && row[tj].is_numeric() {
    num.push(row[tj]);
    row[tj] = '.';
    tj += 1;
  }
  match num.parse::<i32>() {
    Ok(res) => {
      return res;
    },
    Err(_) => return 0
  }
}

fn find_surrounding_numbers(data: &mut Vec<Vec<char>>, i: usize, j: usize, numbers: &mut Vec<i32>) {
  let neighbors = vec![
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1)
  ];

  for (di, dj) in neighbors {
    let ni = i as i32 + di;
    let nj = j as i32 + dj;
    if ni >= 0 && nj >= 0 && nj < data.len() as i32 {
      if let Some(row) = data.get_mut(ni as usize) {
        if let Some(cell) = row.get(nj as usize) {
          if cell.is_numeric() {
            numbers.push(retreive_number(row, nj as usize));
          }
        }
      }
    }
  }
}

fn process_2d_array(data: &mut Vec<Vec<char>>) {
  let mut sum = 0;
  let mut numbers: Vec<i32> = vec![];

  for i in 0..=data.len() - 1 {
    for j in 0..=data[i].len() - 1 {
      if data[i][j] == '*' {
        find_surrounding_numbers(data, i, j, &mut numbers);
        if numbers.len() == 2 {
          sum += numbers[0] * numbers[1];
        }
        numbers.clear();
      }
    }
  }
  println!("Sum: {:?}", sum);
}

pub fn run() {
  let filepath = "src/inputs/3input.txt";
  let mut content: Vec<Vec<char>> = vec![];
  let lines = utility::get_file_lines(filepath);
  
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
  process_2d_array(&mut content);
}