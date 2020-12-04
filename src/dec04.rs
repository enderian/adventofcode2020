use std::fs;
use std::env;
use regex::Regex;

fn main() {
    let extra_star = env::args().any(|s| s == "--extra");
    let input = fs::read_to_string("inputs/dec04.txt").expect("No input!");
    let passports: Vec<&str> = input.split("\n\n").collect();
    let required_props: Vec<&str> = vec!(
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
    );

    let mut valid = 0;
    'outer: for passport in passports {
        let properties: Vec<Vec<&str>> = passport
            .split_whitespace()
            .map(|x| x.split(":").collect())
            .collect();

        for prop in &required_props {
            if !properties.iter().map(|x| x[0]).any(|x| &x == prop) {
                continue 'outer;
            }
        }
        if extra_star {
            if !valid_passport(&properties) {
                continue 'outer;
            }
        }
        valid += 1;
    }

    println!("Valid passports: {}", valid);
}

fn valid_passport(properties: &Vec<Vec<&str>>) -> bool {
    for property in properties {
        let name = property[0];
        let val = property[1];
        println!("\"{}\": \"{}\"", name, val);

        match name {
            "byr" => {
                let p: u32 = val.parse().unwrap();
                if p < 1920 || p > 2002 {
                    return false;
                }
            }
            "iyr" => {
                let p: u32 = val.parse().unwrap();
                if p < 2010 || p > 2020 {
                    return false;
                }
            }
            "eyr" => {
                let p: u32 = val.parse().unwrap();
                if p < 2020 || p > 2030 {
                    return false;
                }
            }
            "hgt" => {
                let cm = Regex::new(r"^(\d+)cm$").unwrap();
                let inc = Regex::new(r"^(\d+)in$").unwrap();
                if cm.is_match(val) {
                    let h: u32 = (&cm.captures(val).unwrap()[1]).parse().unwrap();
                    if h < 150 || h > 193 {
                        return false;
                    }
                } else if inc.is_match(val) {
                    let h: u32 = (&inc.captures(val).unwrap()[1]).parse().unwrap();
                    if h < 59 || h > 76 {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            "hcl" => {
                let regex = Regex::new(r"^#[0-9abcdef]{6}$").unwrap();
                if !regex.is_match(val) {
                    return false;
                }
            }
            "ecl" => {
                let regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
                if !regex.is_match(val) {
                    return false;
                }
            }
            "pid" => {
                let regex = Regex::new(r"^\d{9}$").unwrap();
                if !regex.is_match(val) {
                    return false;
                }
            }
            &_ => {}
        }
    }
    return true;
}