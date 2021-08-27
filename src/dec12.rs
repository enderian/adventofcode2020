use std::fs;
use std::env;
use std::f64;
use regex::Regex;

fn main() {
  let _extra_star = env::args().any(|s| s == "--extra");
  let input = fs::read_to_string("inputs/dec12.txt").expect("No input!");

  let regex = Regex::new(r"^(\w)(\d+)$").unwrap();
  let mut x = 0;
  let mut y = 0;
  let mut dx = 1;
  let mut dy = 0;

  for dir in input.split_whitespace().map(|x| regex.captures(x).unwrap()) {
    let unit: i32 = (&dir)[2].parse().unwrap();
    match &dir[1] {
      "N" => { y += unit },
      "S" => { y -= unit },
      "W" => { x += unit },
      "E" => { x -= unit },
      "R" => {
        match unit {
          90 => { dx }
        };
      },
      "L" => { 

       },
      "F" => {
        y += unit * dy;
        x += unit * dx;
      },
      _ => panic!("Invalid instruction.")
    };
    facing = if facing < 0 { 360 + facing } else { facing };
    println!("{}{} {} {} {}", &dir[1], &dir[2], x, y, facing);
  }
  println!("{} {} {}", x, y, x.abs() + y.abs());
}

fn rotate(x: &mut i32, y: &mut i32, amount: i32) {
  if amount < 0 {
    let t = x;
    y = &(*x * -1 as i32);
    x = y;
  }
}