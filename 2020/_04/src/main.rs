use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_while};
use nom::character::complete::{line_ending, space1};
use nom::combinator::{eof, map};
use nom::multi::many_till;
use nom::sequence::{pair, preceded, separated_pair, terminated};
use std::collections::HashMap;
use std::io::Read;
use util::Result;

fn main() -> Result<()> {
    let mut buf = String::new();
    let mut file = std::fs::File::open("_04/input.txt")?;
    file.read_to_string(&mut buf)?;
    let docs = parse_docs(&buf)?;
    let required_fields = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let valid_documents_1 = docs
        .iter()
        .filter(|doc| {
            required_fields
                .iter()
                .all(|required_field| doc.contains_key(required_field))
        })
        .count();

    let valid_documents_2 = docs
        .iter()
        .filter(|doc| {
            required_fields
                .iter()
                .all(|required_field| doc.contains_key(required_field))
                && doc.iter().all(|(key, value)| validate_field(key, value))
        })
        .count();
    println!("Problem 1, valid docs: {}", valid_documents_1);
    println!("Problem 2, valid docs: {}", valid_documents_2);
    Ok(())
}

fn validate_field(key: &str, value: &str) -> bool {
    match key {
        "byr" => {
            value.len() == 4
                && value
                    .parse::<u16>()
                    .map(|value| value >= 1920 && value <= 2002)
                    .unwrap_or(false)
        }
        "iyr" => {
            value.len() == 4
                && value
                    .parse::<u16>()
                    .map(|value| value >= 2010 && value <= 2020)
                    .unwrap_or(false)
        }
        "eyr" => {
            value.len() == 4
                && value
                    .parse::<u16>()
                    .map(|value| value >= 2020 && value <= 2030)
                    .unwrap_or(false)
        }
        "hgt" => map::<_, _, _, nom::error::Error<&str>, _, _>(
            pair(
                take_while(|chr: char| chr.is_ascii_digit()),
                alt((tag("cm"), tag("in"))),
            ),
            |(height, uom): (&str, &str)| {
                let height = height.parse::<u16>().unwrap();
                match uom {
                    "cm" => height >= 150 && height <= 193,
                    "in" => height >= 59 && height <= 76,
                    _ => false,
                }
            },
        )(value)
        .map(|(_, result)| result)
        .unwrap_or(false),
        "hcl" => map::<_, _, _, nom::error::Error<&str>, _, _>(
            preceded(tag("#"), take(6_usize)),
            |value: &str| {
                value.chars().all(|chr| {
                    chr.is_ascii_hexdigit() && (chr.is_ascii_digit() || chr.is_lowercase())
                })
            },
        )(value)
        .map(|(_, result)| result)
        .unwrap_or(false),
        "ecl" => map::<_, _, _, nom::error::Error<&str>, _, _>(
            alt((
                tag("amb"),
                tag("blu"),
                tag("brn"),
                tag("gry"),
                tag("grn"),
                tag("hzl"),
                tag("oth"),
            )),
            |_| true,
        )(value)
        .map(|(_, result)| result)
        .unwrap_or(false),
        "pid" => map::<_, _, _, nom::error::Error<&str>, _, _>(
            take_while(|chr: char| chr.is_ascii_digit()),
            |pid: &str| pid.len() == 9,
        )(value)
        .map(|(_, result)| result)
        .unwrap_or(false),
        "cid" => true,
        _ => false,
    }
}

fn parse_docs(input: &str) -> std::result::Result<Vec<HashMap<&str, &str>>, &'static str> {
    let (_, (passports, _)) = many_till::<_, _, _, nom::error::Error<&str>, _, _>(
        map(
            many_till(
                terminated(
                    separated_pair(
                        take(3_usize),
                        tag(":"),
                        take_while(|chr: char| {
                            chr.is_ascii_alphanumeric() || chr.is_ascii_punctuation()
                        }),
                    ),
                    alt((space1, line_ending)),
                ),
                alt((line_ending, eof)),
            ),
            |(items, _)| items.into_iter().collect::<HashMap<_, _>>(),
        ),
        eof,
    )(input)
    .map_err(|_| "Failed to parse docs: {:?}")?;
    Ok(passports)
}
