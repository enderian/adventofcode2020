use std::fs;
use std::env;
use std::str::Chars;
use std::collections::HashSet;
use std::collections::HashMap;

fn count_questions(questions: &str) -> usize {
  let questions = questions.split_whitespace().map(|x| x.chars()).flatten();
  let mut set = HashSet::new();
  for q in questions {
    set.insert(q);
  }
  return set.len();
}

fn count_questions_same_answer(questions: &str) -> usize {
  let ans: Vec<Chars> = questions.split_whitespace().map(|x| x.chars()).collect();
  let people = ans.len();
  let mut map: HashMap<char, usize> = HashMap::new();

  for person_answers in ans {
    for a in person_answers {
      match map.get(&a).cloned() {
        Some(c) => &map.insert(a, c + 1),
        None => &map.insert(a, 1)
      };
    }
  }
  return map.iter().filter(|(_k, v)| *v == &people).count();
}

fn main() {
  let extra_star = env::args().any(|s| s == "--extra");
  let input = fs::read_to_string("inputs/dec06.txt").expect("No input!");
  let groups = input.split("\n\n");

  if extra_star {
    let res: usize = groups.map(|g| count_questions_same_answer(g)).sum();
    println!("Answer: {}", res);
  } else {
    let res: usize = groups.map(|g| count_questions(g)).sum();
    println!("Answer: {}", res);
  }
}