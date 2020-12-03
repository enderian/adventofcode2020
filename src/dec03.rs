use std::fs;
use std::env;
use std::str::FromStr;
use std::num::ParseIntError;

struct Forest {
    trees: Vec<Vec<bool>>
}

impl Forest {
    fn count_encounters(&self, down: usize, right: usize) -> u64 {
        let mut x = 0;
        let mut trees: u64 = 0;
        for (ry, row) in self.trees.iter().enumerate() {
            if ry % down != 0 {
                continue;
            }
            for (rx, col) in row.iter().enumerate() {
                if *col && x == rx {
                    trees += 1
                }
            }
            x = (x + right) % row.len();
        }
        return trees;
    }
}

impl FromStr for Forest {
    // Non descriptive type, but meh ¯\_(ツ)_/¯
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees: Vec<Vec<bool>> = s.split_whitespace()
            .map(|s| s.chars().map(|t| t == '#').collect())
            .collect();
        Ok(Forest{trees: trees})
    }
}

fn main() {
    let extra_star = env::args().any(|s| s == "--extra");
    let input = fs::read_to_string("inputs/dec03.txt").expect("No input!");
    let forest = Forest::from_str(&input).expect("Error parsing forest!");

    if extra_star {
        let mut prod: u64 = 1;
        prod *= forest.count_encounters(1, 1);
        prod *= forest.count_encounters(1, 3);
        prod *= forest.count_encounters(1, 5);
        prod *= forest.count_encounters(1, 7);
        prod *= forest.count_encounters(2, 1);
        println!("Product: {}", prod);
    } else {
        println!("Trees: {}", forest.count_encounters(1, 3));
    }
}
