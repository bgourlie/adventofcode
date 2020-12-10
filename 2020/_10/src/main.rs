use util::{read_lines, Result};

fn main() -> Result<()> {
    let mut ratings = read_lines("_10/input.txt")?
        .map(|rating| rating.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    ratings.push(0);
    ratings.sort_unstable();
    ratings.push(ratings.last().unwrap() + 3);

    let (mut ones, mut threes) = (0_usize, 0_usize);
    ratings
        .iter()
        .zip(&ratings[1..])
        .for_each(|(cur, next)| match next - cur {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        });
    println!("{}", ones * threes);
    Ok(())
}
