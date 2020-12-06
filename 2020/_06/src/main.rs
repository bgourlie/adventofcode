use std::collections::HashSet;
use std::iter::FromIterator;

fn main() -> util::Result<()> {
    let mut items = vec![HashSet::<char>::default()];
    for line in util::read_lines("_06/input.txt")? {
        if line == "" {
            items.push(HashSet::default());
        } else {
            line.chars().for_each(|chr| {
                items.last_mut().unwrap().insert(chr);
            });
        }
    }

    let answer1 = items.into_iter().map(|items| items.len()).sum::<usize>();
    println!("Problem one: {}", answer1);

    let mut items = vec![Vec::<HashSet<char>>::default()];
    for line in util::read_lines("_06/input.txt")? {
        if line == "" {
            items.push(Vec::default());
        } else {
            items
                .last_mut()
                .unwrap()
                .push(HashSet::from_iter(line.chars()))
        }
    }

    let answer2 = items
        .into_iter()
        .map(|group| {
            let mut set = HashSet::from(group[0].clone());
            for member in group {
                set.clone().symmetric_difference(&member).for_each(|diff| {
                    set.remove(diff);
                });
            }
            set.len()
        })
        .sum::<usize>();
    println!("Problem 2: {}", answer2);
    Ok(())
}
