use std::fs;
use std::env;
use std::convert::TryInto;
use regex::Regex;

fn main() {
    let extra_star = env::args().any(|s| s == "--extra");

    let contents = fs::read_to_string("inputs/dec02.txt").expect("No input!");
    let passwords: Vec<&str> = contents.split("\n").collect();
    let regex = Regex::new(r"^(\d+)-(\d+)\s(\w):\s(.+)$").unwrap();
    let mut valid = 0;

    for password in passwords.iter() {
        if extra_star {
            if valid_password_extra(&regex, password) {
                valid += 1;
            }
        } else {
            if valid_password(&regex, password) {
                valid += 1;
            }
        }
    }

    println!("Valid passwords: {}", valid);
}

fn valid_password(regex: &Regex, password: &str) -> bool {
    let cap = regex.captures(password).expect("Password did not match regex.");
    let min_occur: i32 = (&cap[1]).parse().unwrap();
    let max_occur: i32 = (&cap[2]).parse().unwrap();
    let expected = (&cap[3]).chars().next().unwrap();

    let mut occur = 0;
    for c in (&cap[4]).chars() {
        if c == expected {
            occur += 1;
        }
    }

    return occur >= min_occur && occur <= max_occur;
}

fn valid_password_extra(regex: &Regex, password: &str) -> bool {
    let cap = regex.captures(password).expect("Password did not match regex.");
    let pos1: i32 = (&cap[1]).parse().unwrap();
    let pos2: i32 = (&cap[2]).parse().unwrap();
    let expected = (&cap[3]).chars().next().unwrap();

    let mut valid = false;
    for (i, c) in (&cap[4]).chars().enumerate() {
        let pos: i32 = i.try_into().unwrap();
        if c == expected && (pos1 == (pos+1) || pos2 == (pos+1)) {
            valid = !valid;
        }
    }
    return valid;
}
