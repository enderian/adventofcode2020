use std::fs;
use std::env;

fn main() {
    let extra_star = env::args().any(|s| s == "--extra");
    let contents = fs::read_to_string("inputs/dec01.txt").expect("No input!");
    let mut numbers: Vec<i32> = contents.
        split_whitespace().
        map(|s| s.parse().expect("Not a number!")).
        collect();

    numbers.sort();
    if extra_star {
        solve_for_extra_star(&numbers);
        return;
    }

    let mut v1 = 0;
    let mut v2 = numbers.len() - 1;
    loop {
        let curr_sum = numbers[v1] + numbers[v2];
        if curr_sum == 2020 {
            // PROFIT
            println!("{} x {} = {}",
                numbers[v1], numbers[v2],
                numbers[v1] * numbers[v2]);
            return;
        } else if curr_sum > 2020 {
            v2 -= 1;
        } else {
            v1 += 1;
        }
    }
}

fn solve_for_extra_star(numbers: &Vec<i32>) {
    for (i, n1) in numbers.iter().enumerate() {
        let mut v1 = i + 1;
        let mut v2 = numbers.len() - 1;
        loop {
            let curr_sum = n1 + numbers[v1] + numbers[v2];
            if curr_sum == 2020 {
                // PROFIT
                println!("{} x {} x {} = {}",
                    n1, numbers[v1], numbers[v2],
                    n1 * numbers[v1] * numbers[v2]);
                return;
            } else if v2 != 0 && curr_sum > 2020 {
                v2 -= 1;
            } else if v1 != numbers.len() - 1 {
                v1 += 1;
            } else {
                break;
            }
        }
    }
}