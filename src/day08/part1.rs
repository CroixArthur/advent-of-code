/*
use std::collections::{HashMap, HashSet};

use crate::utility;

fn load_values(content: String, values: &mut HashMap<String, (String, String)>) {
  let header: Vec<&str> = content.split(" = ").collect();
  let tuple: Vec<&str> = (&header[1][1..header[1].len() - 1]).split(", ").collect();

  values.insert(header[0].to_owned(), (tuple[0].to_owned(), tuple[1].to_owned()));
}

// fn load_value(content: String, values: &mut Vec<(String, (String, String))>) {
//   let header: Vec<&str> = content.split(" = ").collect();
//   let tuple: Vec<&str> = (&header[1][1..header[1].len() - 1]).split(", ").collect();

//   values.push((header[0].to_owned(), (tuple[0].to_owned(), tuple[1].to_owned())));
// }

// fn follow_instructions(values: &Vec<(String, (String, String))>, instructions: String) -> i32 {
//   let mut step = 0;
//   let mut curr_index = 0;
//   let mut instr_iter = instructions.chars();

//   while curr_index < values.len() && values[curr_index].0 != "ZZZ" {
//     match instr_iter.next() {
//       Some(instr) => {
//         curr_index = match instr {
//           'R' => {
//             step += 1;
//             values.iter().position(|val| val.0 == values[curr_index].1.1).unwrap()
//           },
//           'L' => {
//             step += 1;
//             values.iter().position(|val| val.0 == values[curr_index].1.0).unwrap()
//           }
//           _ => curr_index
//         };
//       },
//       None => instr_iter = instructions.chars()
//     }
//   }

//   step
// }

fn get_values(map: &HashMap<String, (String, String)>, visited: &mut HashMap<String, (String, String)>, curr_val: &String) {
  if curr_val.eq("LRV") { println!("VICTOIRE !!!!"); }
  println!("Value: {}: {:?}", curr_val, map[curr_val]);
  if !visited.contains_key(curr_val) {
    visited.insert(curr_val.to_string(), (map[curr_val].0.to_string(), map[curr_val].1.to_string()));
    get_values(map, visited, &map[curr_val].0);
    get_values(map, visited, &map[curr_val].1);
  }
}

// fn get_possible_outcomes(map: &HashMap<String, (String, String)>) {
  // let mut new_map: HashMap<String, (String, String)> = HashMap::new();
  // let curr_val = "ZZZ".to_owned();
// 
  // get_values(map, &mut new_map, &curr_val);
  // println!("new_map: {:#?}, new_map_len: {}, map_len: {}", new_map, new_map.len(), map.len());
// }

fn get_possible_outcomes(map: &HashMap<String, (String, String)>) {
  let mut visited: HashSet<String> = HashSet::new();
  let mut stack: Vec<String> = vec!["ZZZ".to_owned()];

  while let Some(curr_val) = stack.pop() {
    println!("{curr_val}");
    if curr_val == "LRV" { println!("VICTOIRE !!!!"); }
    if !visited.contains(&curr_val) {
      visited.insert(curr_val.clone());
      let (left, right) = &map[&curr_val];
      stack.push(right.to_owned());
      stack.push(left.to_owned());
    }
  }
  // println!("visited: {:?}, visited_len: {}, map_len: {}", visited, visited.len(), map.len());
}

fn follow_instructions(map: &HashMap<String, (String, String)>, first_val: String, instructions: String) -> i32 {
  let mut step = 0;
  let mut curr_val: &str = first_val.as_str();
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
  let filepath = "src/inputs/8input.txt";
  let mut instructions: String = "".to_owned();
  let mut map: HashMap<String, (String, String)> = HashMap::new();
  // let mut values: Vec<(String, (String, String))> = vec![];
  let mut first_val: String = "".to_string();

  match utility::get_file_lines(filepath) {
    Ok(value) => {
      for (i, line) in value.enumerate() {
        if let Ok(res) = line {
          if i == 0 {
            instructions = res;
          } else if i != 1 {
            // load_value(res, &mut values);
            if i == 2 {
              // println!("Res: {res}");
              first_val = res.get(0..3).unwrap().to_owned();
            }
            load_values(res, &mut map);
          }
        }
      }
    }
    Err(e) => println!("{e}")
  }
  // println!("Map: {:#?}\nInstructions: {:?}", map, instructions);

  get_possible_outcomes(&map);
  // println!("Steps: {}", follow_instructions(&values, instructions));
  // println!("Steps: {}", follow_instructions(&map, first_val, instructions));
}
*/

pub fn run() {
  
}