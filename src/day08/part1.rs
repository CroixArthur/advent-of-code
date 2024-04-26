use std::collections::HashMap;

use crate::utility;

fn load_values(content: String, values: &mut HashMap<String, (String, String)>) {
  let header: Vec<&str> = content.split(" = ").collect();
  let tuple: Vec<&str> = (&header[1][1..header[1].len() - 1]).split(", ").collect();

  values.insert(header[0].to_owned(), (tuple[0].to_owned(), tuple[1].to_owned()));
}

fn follow_instructions(map: &HashMap<String, (String, String)>, instructions: String) -> i32 {
  let mut step = 0;
  let mut curr_val: &str = "AAA";
  let mut instr_iter = instructions.chars();

  while curr_val != "ZZZ" {
    match instr_iter.next() {
      Some(instr) => {
        curr_val = match instr {
          'R' => {
            step += 1;
            map[curr_val].1.as_str()
          },
          'L' => {
            step += 1;
            map[curr_val].0.as_str()
          }
          _ => curr_val
        };
      },
      None => instr_iter = instructions.chars()
    }
  }

  step
}

pub fn run() {
  let filepath = "src/input.txt";
  let mut instructions: String = "".to_owned();
  let mut map: HashMap<String, (String, String)> = HashMap::new();

  match utility::get_file_lines(filepath) {
    Ok(value) => {
      for (i, line) in value.enumerate() {
        if let Ok(res) = line {
          if i == 0 {
            instructions = res;
          } else if i > 1 {
            load_values(res, &mut map);
          }
        }
      }
    }
    Err(e) => println!("{e}")
  }

  println!("Steps: {}", follow_instructions(&map, instructions));
}