use std::cmp::Ordering;
use std::time::Instant;
use util::{read_lines, Result};

fn main() -> Result<()> {
    let numbers = read_lines("_09/input.txt")?
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let start = Instant::now();
    let answer1 = solve_first(&numbers);
    let answer2 = solve_second(&numbers, answer1);
    let duration = Instant::now() - start;
    println!("answer 1: {}", answer1);
    println!("answer 2: {}", answer2);
    println!("duration: {}ns", duration.as_nanos());
    Ok(())
}

fn solve_first(numbers: &[i64]) -> i64 {
    let mut i = 25;
    loop {
        if i == numbers.len() {
            panic!("No solution found");
        }
        let mut v = numbers[(i - 25)..i].iter().copied().collect::<Vec<i64>>();
        v.sort_unstable();
        v.dedup();

        let (mut bottom, mut top) = (0_usize, v.len() - 1);
        let found = loop {
            if top - bottom < 1 {
                break None;
            }
            let (low, high) = (v[bottom], v[top]);
            match numbers[i].cmp(&(low + high)) {
                Ordering::Greater => bottom += 1,
                Ordering::Equal => break Some(low + high),
                Ordering::Less => top -= 1,
            }
        };

        if found.is_none() {
            break numbers[i];
        } else {
            i += 1;
        }
    }
}

fn solve_second(numbers: &[i64], to_find: i64) -> i64 {
    let (mut low, mut high) = (0, 2);
    loop {
        let sum = numbers[low..high].iter().sum::<i64>();
        match sum.cmp(&to_find) {
            Ordering::Less => high += 1,
            Ordering::Equal => {
                let mut sorted = numbers[low..high].iter().copied().collect::<Vec<i64>>();
                sorted.sort_unstable();
                break sorted.first().unwrap() + sorted.last().unwrap();
            }
            Ordering::Greater => {
                low += 1;
                high = low + 2;
            }
        }
    }
}
