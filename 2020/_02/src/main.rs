use nom::bytes::complete::{tag, take, take_while};
use nom::character::complete::newline;
use nom::character::{is_alphanumeric, is_digit};
use nom::combinator::{eof, map, map_res};
use nom::multi::many_till;
use nom::sequence::{separated_pair, terminated, tuple};
use nom::IResult;
use std::io::Read;
use std::ops::BitXor;
use util::Result;

#[derive(Debug)]
struct Entry<'a> {
    required_letter: char,
    required_occurrences: (u32, u32),
    password: &'a [u8],
}

impl<'a> Entry<'a> {
    fn is_valid_solution_1(&self) -> bool {
        let (min_occur, max_occur) = self.required_occurrences;
        let mut occurs = 0;

        for chr in self.password.iter().copied().map(|chr| chr as char) {
            if chr == self.required_letter {
                occurs += 1;
                if occurs > max_occur {
                    return false;
                }
            }
        }

        occurs >= min_occur
    }

    fn is_valid_solution_2(&self) -> Result<bool> {
        let (pos_1, pos_2) = self.required_occurrences;
        let char1_match = self
            .password
            .get(pos_1 as usize - 1)
            .copied()
            .map(|chr| (chr as char) == self.required_letter)
            .ok_or("Position out of bounds")?;

        let char2_match = self
            .password
            .get(pos_2 as usize - 1)
            .copied()
            .map(|chr| (chr as char) == self.required_letter)
            .ok_or("Position out of bounds")?;
        Ok(char1_match.bitxor(char2_match))
    }
}

fn main() -> Result<()> {
    let mut buf = Vec::new();
    let mut file = std::fs::File::open("_02/input.txt")?;
    file.read_to_end(&mut buf)?;
    let entries = parse_entries(&buf)?;
    let valid_entries_solution_1 = entries
        .iter()
        .map(|entry| if entry.is_valid_solution_1() { 1 } else { 0 })
        .sum::<u32>();

    let valid_entries_solution_2 = entries
        .iter()
        .map(|entry| {
            if entry.is_valid_solution_2().unwrap() {
                1
            } else {
                0
            }
        })
        .sum::<u32>();
    println!("Solution 1: {} valid entries", valid_entries_solution_1);
    println!("Solution 2: {} valid entries", valid_entries_solution_2);
    Ok(())
}

fn parse_entries(input: &[u8]) -> std::result::Result<Vec<Entry>, &'static str> {
    fn parse_number(input: &[u8]) -> IResult<&[u8], u32> {
        map_res(take_while(is_digit), |bytes: &[u8]| {
            std::str::from_utf8(bytes)
                .map_err(|_| "Failed to convert bytes to string")
                .and_then(|digits| digits.parse::<u32>().map_err(|_| "Failed to parse number"))
        })(input)
    }

    fn parse_range(input: &[u8]) -> IResult<&[u8], (u32, u32)> {
        terminated(
            separated_pair(parse_number, tag("-"), parse_number),
            tag(" "),
        )(input)
    }

    fn parse_required_letter(input: &[u8]) -> IResult<&[u8], char> {
        map_res(terminated(take(1_usize), tag(": ")), |chr: &[u8]| {
            chr.first().copied().map(|chr| chr as char).ok_or(())
        })(input)
    }

    let (_, (entries, _)) = many_till(
        map(
            terminated(
                tuple((
                    parse_range,
                    parse_required_letter,
                    take_while(is_alphanumeric),
                )),
                newline,
            ),
            |(required_occurrences, required_letter, password)| Entry {
                required_occurrences,
                required_letter,
                password,
            },
        ),
        eof,
    )(input)
    .map_err(|_| "Failed to parse entries")?;

    Ok(entries)
}
