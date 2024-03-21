use crate::utility;

use super::Hailstone;

fn calculate_intersection(hs1: &Hailstone, hs2: &Hailstone) -> (f64, f64, f64, f64) {
  let t1 = ((hs2.px - hs1.px) * hs2.vy - (hs2.py - hs1.py) * hs2.vx) as f64 / (hs1.vx * hs2.vy - hs1.vy * hs2.vx) as f64;
  let t2 = ((hs2.px - hs1.px) * hs1.vy - (hs2.py - hs1.py) * hs1.vx) as f64 / (hs1.vx * hs2.vy - hs1.vy * hs2.vx) as f64;
  let position = ((hs1.px + hs1.vx * t1) as f64, (hs1.py + hs1.vy * t1) as f64);
  return (position.0, position.1, t1, t2);
}

fn handle_intersection(hs1: &Hailstone, hs2: &Hailstone, min_area: i64, max_area: i64, sum: &mut i32) -> bool {
  println!("Hailstone A: {:?}, {:?}, {:?} @ {:?}, {:?}, {:?}", hs1.px, hs1.py, hs1.pz, hs1.vx, hs1.vy, hs1.vz);
  println!("Hailstone B: {:?}, {:?}, {:?} @ {:?}, {:?}, {:?}", hs2.px, hs2.py, hs2.pz, hs2.vx, hs2.vy, hs2.vz);

  if (hs1.vx * hs2.vy - hs2.vx * hs1.vy) as f64 == 0.0 {
    println!("Hailstones' paths are parallel; they never intersect.");
    return false;
  }

  let result = calculate_intersection(hs1, hs2);

  if result.2 < 1.0 && result.3 < 1.0 {
    println!("Hailstones' paths crossed in the past for both hailstones.");
  } else if result.2 < 1.0 {
    println!("Hailstones' paths crossed in the past for hailstone A.");
  } else if result.3 < 1.0 {
    println!("Hailstones' paths crossed in the past for hailstone B.");
  } else if result.0 >= min_area as f64 && result.0 <= max_area as f64
    && result.1 >= min_area as f64 && result.1 <= max_area as f64 {
    *sum += 1;
    println!("Hailstones' paths will cross inside the test area (at x={}, y={})", result.0, result.1);
  } else {
    println!("Hailstones' paths will cross outside the test area (at x={}, y={})", result.0, result.1);
  }
  true
}

fn get_values(string: &str) -> Vec<i64> {
  let mut res: Vec<i64> = vec![];

  string.split(", ").for_each(|val| {
    match val.parse::<i64>() {
      Ok(num) => res.push(num),
      Err(_) => ()
    }
  });
  res
}

pub fn run() {
  let mut hailstones: Vec<Hailstone> = vec![];
  let min_area = 200000000000000;
  let max_area = 400000000000000;
  let mut sum: i32 = 0;

  match utility::get_file_array("src/inputs/24input.txt") {
    Ok(res) => {
      for line in res {
        let s = line.into_iter().collect::<String>();
        let elems: Vec<&str> = s.split(" @ ").collect();
        let position = get_values(elems[0]);
        let velocity = get_values(elems[1]);

        hailstones.push(Hailstone {
          px: position[0] as f64,
          py: position[1] as f64,
          pz: position[2] as f64,
          vx: velocity[0] as f64,
          vy: velocity[1] as f64,
          vz: velocity[2] as f64
        });
      }
      for i in 0..hailstones.len() {
        for j in (i + 1)..hailstones.len() {
          handle_intersection(&hailstones[i], &hailstones[j], min_area, max_area, &mut sum);
        }
      }
    },
    Err(e) => {
      panic!("{e}");
    }
  }
  println!("Sum: {}", sum);
}
