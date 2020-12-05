use util::{read_lines, Result};

fn main() -> Result<()> {
    let mut ids = read_lines("_05/input.txt")?
        .map(|line| bsp(&line[..7]) * 8 + bsp(&line[7..]))
        .collect::<Vec<_>>();

    ids.sort_unstable();

    let my_seat = ids
        .iter()
        .copied()
        .enumerate()
        .find(|(i, id)| *id != i + ids[0])
        .map(|(_, id)| id - 1)
        .unwrap();

    println!("answer 1: {}", ids.iter().max().unwrap());
    println!("answer 2: {}", my_seat);
    Ok(())
}

fn bsp(input: &str) -> usize {
    input.chars().enumerate().fold(0, |acc, (i, chr)| {
        if chr == 'B' || chr == 'R' {
            acc + 2_usize.pow((input.len() - i - 1) as u32)
        } else {
            acc
        }
    })
}
