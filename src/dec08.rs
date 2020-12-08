use std::fs;
use std::env;
use std::collections::HashSet;

fn parse_instruction(s: &str) -> Vec<i16> {
  let mut split = s.split(" ");
  let instruction = match &split.next() {
    Some("nop") => -1,
    Some("acc") => 0,
    Some("jmp") => 1,
    &x => panic!("invalid instruction {:?}", x),
  };
  let arg = split.next().unwrap().parse().unwrap();
  return vec![instruction, arg];
}

fn main() {
  let extra_star = env::args().any(|s| s == "--extra");
  let input = fs::read_to_string("inputs/dec08.txt").expect("No input!");
  let program: Vec<Vec<i16>> = input.split("\n").map(|i| parse_instruction(i)).collect();
  let mut test_instruction = if extra_star { 0 } else { -1 };

  loop {
    let mut ip: i16 = 0;
    let mut acc: i16 = 0;
    let mut visited = HashSet::new();

    loop {
      if program.len() <= ip as usize {
        println!("Program exited! Acc: {}", acc);
        return;
      }
      let n = &program[ip as usize];
      if visited.contains(&ip) {
        println!("Loop detected! Acc: {}", acc);
        break;
      }
      visited.insert(ip);
      match if test_instruction == ip { &n[0] * -1 } else { n[0] } {
        -1 => { ip += 1; },
        0 => { acc += &n[1]; ip += 1; }
        1 => { ip += &n[1]; }
        _ => {}
      };
    }

    if extra_star {
      test_instruction += 1;
    } else {
      return;
    }
  }
}