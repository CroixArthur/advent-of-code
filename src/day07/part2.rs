use std::cmp::Ordering;

use crate::utility;

fn compare_chars(fst: char, snd: char) -> Ordering {
  let values = vec!['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

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

fn get_card_occurences(hand: &str) -> Vec<(char, i8)> {
  let mut known_chars: Vec<char> = vec![];
  let mut card_occurences: Vec<(char, i8)> = vec![];
  let mut joker_occ = 0;
  
  for card in hand.chars() {
    if !known_chars.contains(&card) {
      card_occurences.push((card, hand.chars().filter(|&c| c == card).count() as i8));
      known_chars.push(card);
    }
  }
  if card_occurences.len() == 1 {
    return card_occurences;
  }
  for (i, occ) in card_occurences.iter().enumerate() {
    if occ.0 == 'J' {
      joker_occ = occ.1;
      card_occurences.remove(i);
      break;
    }
  }
  card_occurences.sort_by(|occ1, occ2| {
    if occ1.1 < occ2.1 { return Ordering::Greater }
    else if occ1.1 > occ2.1 { return Ordering::Less }
    else { return Ordering::Equal }
  });
  if let Some(top) = card_occurences.first_mut() {
    top.1 += joker_occ;
  }
  card_occurences
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
  let card_occurences: Vec<(char, i8)> = get_card_occurences(hand);

  for occurence in card_occurences {
    if occurence.1 == 2 {
      if max_val == 1 {
        return 2;
      } else if max_val == 3 {
        return 4;
      }
      max_val = 1;
    } else if occurence.1 == 3 {
      if max_val == 1 {
        return 4;
      } else {
        max_val = 3;
      }
    } else if occurence.1 > 3 {
      return occurence.1 + 1;
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