use std::fs;
use std::env;
use std::cmp;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, Copy, Clone)]
struct BoardingPass {
  row: u32,
  col: u32
}

impl BoardingPass {

  fn id(&self) -> u32 {
    return self.row * 8 + self.col;
  }
}

impl FromStr for BoardingPass {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut ymin = 0; let mut ymax = 128;
    let mut xmin = 0; let mut xmax = 8;
    for c in s.chars() {
      match c {
        'B' => ymin += cmp::max(2, ymax - ymin) / 2,
        'F' => ymax -= cmp::max(2, ymax - ymin) / 2,
        'R' => xmin += cmp::max(2, xmax - xmin) / 2,
        'L' => xmax -= cmp::max(2, xmax - xmin) / 2,
        _ => {}
      }
    }
    return Ok(BoardingPass{row: ymin, col: xmin})
  }
}

fn main() {
  let extra_star = env::args().any(|s| s == "--extra");
  let input = fs::read_to_string("inputs/dec05.txt").expect("No input!");
  let passes_txt = input.split_whitespace();
  let passes: Vec<BoardingPass> = passes_txt
    .map(|x| BoardingPass::from_str(x).expect("Invalid boarding pass"))
    .collect();

  let mut ids: Vec<u32> = passes.iter().map(|p| p.id()).collect();
  ids.sort();

  if extra_star {
    let mut last_seat = ids[0];
    for id in ids {
      if id - last_seat > 1 {
        println!("Possible seat: {}", id - 1);
      }
      last_seat = id;
    }
  } else {
    println!("Max ID: {}", ids[ids.len() - 1]);
  }
}