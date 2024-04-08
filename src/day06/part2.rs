use crate::utility;

fn get_times_nbr(time: i64, best_distance: i64) -> i64 {
  let mut best_score_occs: i64 = 0;

  for ms in 0..time {
    if ms * (time - ms) > best_distance {
      best_score_occs += 1;
    }
  }
  best_score_occs
}

pub fn run() {
  let filepath = "src/inputs/6input.txt";
  let mut content: Vec<String> = vec![];
  let lines = utility::get_file_lines(filepath);
  let mut times: Vec<i64> = vec![];
  let mut distances: Vec<i64> = vec![];
  let mut prod: i64 = 1;
  
  match lines {
    Ok(value) => {
      for line in value {
        if let Ok(res) = line {
          content.push(res)
        }
      }
    }
    Err(e) => println!("{e}")
  }
  match utility::get_bigints(content[0].split(':').collect::<Vec<&str>>()[1]) {
    Ok(res) => times = res,
    Err(_) => ()
  }
  match utility::get_bigints(content[1].split(':').collect::<Vec<&str>>()[1]) {
    Ok(res) => distances = res,
    Err(_) => ()
  }
  if times.is_empty() || distances.is_empty() {
    return;
  }
  for i in 0..times.len() {
    prod *= get_times_nbr(times[i], distances[i]);
  }
  println!("Prod: {prod}")
}