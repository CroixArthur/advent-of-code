use crate::utility;

use std::{fs::File, io::{BufReader, Lines}, iter::Peekable, sync::{Arc, Mutex}, thread};

fn get_next_element_value(map: &Vec<Vec<i64>>, source: i64) -> i64 {
  for values in map {
    if source >= values[1] && source <= values[1] + values[2] {
      return source + values[0] - values[1];
    }
  }
  source
}

pub fn run() {
  let filepath = "src/inputs/5input.txt";
  let mut seeds: Vec<i64> = vec![];
  let mut maps: Vec<Vec<Vec<i64>>> = vec![];
  let mut iter: Peekable<Lines<BufReader<File>>>;
  let mut type_index: i64 = -1;
  let locations: Arc<Mutex<Vec<i64>>> = Arc::new(Mutex::new(vec![]));
  let mut threads: Vec<thread::JoinHandle<()>> = vec![];
  
  match utility::get_file_lines(filepath) {
    Ok(value) => {
      iter = value.peekable();
      while let Some(line) = iter.next() {
        if let Ok(res) = line {
          if res.starts_with("seeds: ") {
            match utility::get_bigints(&res) {
              Ok(nums) => seeds = nums,
              Err(_) => (),
            }
          } else if !res.is_empty() && res.chars().nth(0).unwrap().is_numeric() {
            match utility::get_bigints(&res) {
              Ok(nums) => {
                if maps.len() == type_index as usize {
                  maps.push(vec![]);
                }
                maps[type_index as usize].push(nums);
              },
              Err(_) => (),
            }
          } else if !res.is_empty() {
            type_index += 1;
          }
        }
      }
    }
    Err(e) => println!("{e}")
  }
  for j in (0..seeds.len()).step_by(2) {
    println!("J: {j}");
    let maps_clone = maps.clone();
    let seeds_clone = seeds.clone();
    let locations_clone = Arc::clone(&locations);
    threads.push(thread::spawn(move || {
      let mut val;
      let mut best_location: Option<i64> = None;
      for seed in seeds_clone[j]..(seeds_clone[j] + seeds_clone[j+1]) {
        println!("Seed: {seed}");
        val = seed;
        for i in 0..7 {
          val = get_next_element_value(&maps_clone[i], val);
        }
        if best_location.is_none() || best_location.is_some_and(|l| l > val) {
          best_location = Some(val);
        }
      }
      let mut shared_locations = locations_clone.lock().unwrap();
      if let Some(loc) = best_location {
        shared_locations.push(loc);
      }
    }));
  }
  for th in threads {
    th.join().unwrap();
  }
  let mut locked_locations = locations.lock().unwrap();
  locked_locations.sort();
  println!("Best location: {:?}", locked_locations[0]);
}