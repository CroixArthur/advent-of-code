use std::cmp::Ordering;

use crate::utility;

fn compare_chars(fst: char, snd: char) -> Ordering {
  let values = vec!['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

  if let Some(fst_pos) = values.iter().position(|&x| x == fst) {
    if let Some(snd_pos) = values.iter().position(|&x| x == snd) {
      if fst_pos < snd_pos {
        return Ordering::Less;
      } else if fst_pos > snd_pos {
        return Ordering::Greater;
      } else {
        return Ordering::Equal;
      }
    }
  }
  Ordering::Equal
}

/**
 * 0: High card
 * 1: One pair
 * 2: Two pair
 * 3: Three of a kind
 * 4: Full house
 * 5: Four of a kind
 * 6: Five of a kind
 */
fn get_hand_value(hand: &str) -> i8 {
  let mut max_val: i8 = 0;
  let mut card_occurences: i8;
  let mut known_chars: Vec<char> = vec![];

  for card in hand.chars() {
    if !known_chars.contains(&card) {
      card_occurences = hand.chars().filter(|&c| c == card).count() as i8;
      known_chars.push(card);
      if card_occurences == 2 {
        if max_val == 1 {
          return 2;
        } else if max_val == 3 {
          return 4;
        }
        max_val = 1;
      } else if card_occurences == 3 {
        if max_val == 1 {
          return 4;
        } else {
          max_val = 3;
        }
      } else if card_occurences > 3 {
        return card_occurences + 1;
      }
    }
  }
  max_val
}

fn compare_hands(hand1: (i8, &str, i32), hand2: (i8, &str, i32)) -> Ordering {
  if hand1.0 < hand2.0 {
    return Ordering::Less;
  } else if hand1.0 > hand2.0 {
    return Ordering::Greater;
  } else {
    for i in 0..hand1.1.len() {
      match compare_chars(hand1.1.as_bytes()[i] as char, hand2.1.as_bytes()[i] as char) {
        Ordering::Greater => return Ordering::Greater,
        Ordering::Less => return Ordering::Less,
        Ordering::Equal => ()
      }
    }
  }
  return Ordering::Equal;
}

fn get_sorted_hands<'a>(sorted: &mut Vec<(i8, &'a str, i32)>, hands: &'a Vec<Vec<String>>) {
  hands.iter().for_each(|hand| sorted.push((get_hand_value(hand[0].as_str()), hand[0].as_str(), hand[1].parse::<i32>().unwrap())));
  sorted.sort_by(|&hand1, &hand2| compare_hands(hand1, hand2));
}

pub fn run() {
  let filepath = "src/input.txt";
  let mut content: Vec<Vec<String>> = vec![];
  let mut sorted_hands: Vec<(i8, &str, i32)> = vec![];
  let mut sum = 0;

  if let Ok(lines) = utility::get_file_lines(filepath) {
    for line in lines.flatten() {
      let split_line: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();
      content.push(split_line);
    }
  }
  get_sorted_hands(&mut sorted_hands ,&content);
  for i in 0..sorted_hands.len() {
    sum += sorted_hands[i].2 * (i as i32 + 1);
  }
  println!("Sum: {}", sum);
}