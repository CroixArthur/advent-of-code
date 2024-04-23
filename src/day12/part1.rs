use crate::utility;

fn can_put_size(line: &Vec<char>, size: i8, index: usize) -> bool {
  if line.get(index..index+(size as usize)).is_some_and(|port| port.iter().all(|&c| c == '#' || c == '?')) {
    let before = line.get((index as i32 - 1) as usize);
    let after = line.get((index as i32 + size as i32) as usize);
    if (before.is_none() || before.is_some_and(|&c| c == '.' || c == '?'))
      && (after.is_none() || after.is_some_and(|&c| c == '.' || c == '?')) {
      return true;
    }
  }

  false
}

// fn find_max_arrangements_nbr(raw: String) -> i32 {
//   let mut arrangements = 0;
//   let mut line: Vec<char>;
//   let mut groups: Vec<i8> = vec![];
//   let mut i = 0;
//   let mut j = 0;
//   let mut offset = 0;

//   if let Some((fst, snd)) = raw.split_once(" ") {
//     line = fst.chars().collect();
//     groups = snd.split(",").map(|nb| nb.parse::<i8>().unwrap()).collect();

//     while i < line.len() {
//       offset += 1;
//       while j < groups.len() && can_put_size(&line, groups[j], i) {
//         println!("{} at i: {i}", groups[j]);
//         i += groups[j] as usize + 1;
//         j += 1;
//       }
//       if j == groups.len() {
//         println!("Arranged");
//         arrangements += 1;
//         j = 0;
//         i = offset;
//       } else {
//         i += 1;
//       }
//     }

//     println!("Line: {:?}", line);
//     println!("Groups: {:?}\n", groups);
//     println!();
//   }

//   arrangements
// }

#[derive(Debug)]
struct SizePos {
  size_pos: usize,
  line_pos: usize
}

impl Clone for SizePos
{
  fn clone(&self) -> Self {
      return SizePos {
        size_pos: self.size_pos,
        line_pos: self.line_pos
      }
  }
}

fn count_gears(line: Vec<char>, known_pos: Vec<SizePos>) -> usize {
  let mut size = 0;

  for mut i in 0..line.len() {
    if line[i] == '#' {
      size += 1;
      while i < line.len() && line[i] == '#' { i += 1; }
    }
  }

  size
}

fn find_max_arrangements_nbr(raw: String) -> usize {
  let mut arrangements: usize = 0;
  let line: Vec<char>;
  let sizes: Vec<i8>;
  let mut i = 0;
  let mut j = 0;
  let mut known_pos: Vec<SizePos> = vec![];

  if let Some((fst, snd)) = raw.split_once(" ") {
    line = fst.chars().collect();
    sizes = snd.split(",").map(|nb| nb.parse::<i8>().unwrap()).collect();

    while i < line.len() {
      if line[i] != '.' {
        if can_put_size(&line, sizes[j], i) {
          if j == sizes.len() - 1 && count_gears(line.clone(), known_pos.clone().into_iter().chain(std::iter::once(SizePos {size_pos: j, line_pos: i})).collect()) == (known_pos.len() + 1) {
            arrangements += 1;
            println!("Current Poses: {:?}", vec![&known_pos, &vec![(SizePos {size_pos: j, line_pos: i})]]);
          } else if j == sizes.len() - 1 {
            match known_pos.pop() {
              Some(val) => {
                i = val.line_pos + 1;
                j = val.size_pos;
              },
              None => break
            }
          } else {
            known_pos.push(SizePos {size_pos: j, line_pos: i});
            i += sizes[j] as usize;
            j += 1;
          }
          // println!("Known Pos: {:?}\nArrangements: {arrangements}\nI: {i}, J: {j}\n", known_pos);
        }
      }
      i += 1;
      if i >= line.len() && !known_pos.is_empty() {
        match known_pos.pop() {
          Some(val) => {
            i = val.line_pos + 1;
            j = val.size_pos;
          },
          None => break
        }
      }
    }

    println!("Line: {:?}", line);
    println!("Sizes: {:?}\n", sizes);
  }
  println!("Arrangements: {arrangements}");

  arrangements
}

pub fn run(filepath: &str) {
  // let filepath = "src/input.txt";
  let mut sum = 0;

  // println!("Size1: {}", if can_put_size(&vec!['#','.','#','.','#','#','#'], 1, 0) { "true" } else { "false" });
  // println!("Size2: {}", if can_put_size(&vec!['#','.','#','.','#','#','#'], 1, 1) { "true" } else { "false" });
  // println!("Size2: {}", if can_put_size(&vec!['?','?','?','.','#','#','#'], 1, 2) { "true" } else { "false" });
  // println!("Size3: {}", if can_put_size(&vec!['?','?','?','.','#','#','#'], 3, 4) { "true" } else { "false" });

  match utility::get_file_lines(filepath) {
    Ok(value) => {
      for line in value {
        if let Ok(res) = line {
          sum += find_max_arrangements_nbr(res);
        }
      }
    }
    Err(e) => println!("{e} with argument {filepath}")
  }

  println!("Sum: {sum}")
}