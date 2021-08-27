use std::fs;
use std::env;

fn main() {
  let extra_star = env::args().any(|s| s == "--extra");
  let input = fs::read_to_string("inputs/dec09.txt").expect("No input!");
  let numbers: Vec<u32> = input.split("\n").unwrap().map(|x| x.parse().unwrap());
  let preample = numbers[:25];

}