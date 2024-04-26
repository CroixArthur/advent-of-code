use std::collections::HashMap;

use crate::utility;

fn load_values(content: String, map: &mut HashMap<String, (String, String)>, values: &mut Vec<String>) {
  let header: Vec<&str> = content.split(" = ").collect();
  let tuple: Vec<&str> = (&header[1][1..header[1].len() - 1]).split(", ").collect();

  map.insert(header[0].to_owned(), (tuple[0].to_owned(), tuple[1].to_owned()));
  if header[0].chars().last().unwrap() == 'A' {
    values.push(header[0].to_string());
  }
}

fn aled_instruction(map: &HashMap<String, (String, String)>, instructions: &String, value: &str, known_values: &mut Vec<(String, i8, i32)>) {
  let mut step = 0;
  let mut curr_val: &str = value;
  let mut instr_iter = instructions.chars().enumerate();

  while curr_val.chars().last().unwrap() != 'Z' {
    match instr_iter.next() {
      Some(instr) => {
        curr_val = match instr.1 {
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
        if curr_val.chars().last().unwrap() == 'Z' {
          println!("{:?}", (curr_val, instr.0, step));
          known_values.push((curr_val.to_owned(), instr.0 as i8, step));
          break;
        }
      },
      None => instr_iter = instructions.chars().enumerate()
    }
  }
}

fn get_matching_step(values: &Vec<(String, i8, i32)>) -> u128 {
  let mut index = 1;
  let mut mult = 1;
  let mut temp_mult: u128 = 1;

  while let Some(val) = values.get(index) {
    if (values[0].2 as u128 * mult) % (val.2 as u128) != 0 {
      while (values[0].2 as u128 * mult) % (val.2 as u128) != 0 {
        if index > 1 {
          mult += temp_mult
        } else {
          mult += 1
        }
      }
      index = 1;
    } else {
      index += 1;
    }
    if temp_mult == 1 {
      temp_mult = mult;
    }
  }

  values[0].2 as u128 * mult
}

fn follow_instructions(map: &HashMap<String, (String, String)>, instructions: String, values: Vec<String>) -> u128 {
  let mut known_values: Vec<(String, i8, i32)> = vec![];

  for value in &values {
    aled_instruction(map, &instructions, value, &mut known_values);
  }

  get_matching_step(&known_values)
}

pub fn run() {
  let filepath = "src/input.txt";
  let mut instructions: String = "".to_owned();
  let mut map: HashMap<String, (String, String)> = HashMap::new();
  let mut values: Vec<String> = Vec::new();

  match utility::get_file_lines(filepath) {
    Ok(value) => {
      for (i, line) in value.enumerate() {
        if let Ok(res) = line {
          if i == 0 {
            instructions = res;
          } else if i > 1 {
            load_values(res, &mut map, &mut values);
          }
        }
      }
    }
    Err(e) => println!("{e}")
  }

  println!("Steps: {}", follow_instructions(&map, instructions, values));
}