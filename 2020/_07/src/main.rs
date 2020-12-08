use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::newline;
use nom::combinator::{eof, map};
use nom::multi::many_till;
use nom::sequence::{pair, separated_pair, terminated};
use nom::IResult;
use std::collections::HashMap;
use std::io::Read;
use util::Result;

fn main() -> Result<()> {
    let mut buf = String::new();
    let mut file = std::fs::File::open("_07/input.txt")?;
    file.read_to_string(&mut buf)?;
    let bags = parse(&buf)?;
    let num_contains = bags
        .keys()
        .filter(|bag| contains_nested(bag, &bags, "shiny gold"))
        .count();
    println!("solution 1: {}", num_contains);
    println!(
        "solution 2: {}",
        find_total_bags_containing("shiny gold", &bags)
    );
    Ok(())
}

fn find_total_bags_containing(color: &str, bags: &HashMap<&str, HashMap<&str, u32>>) -> u32 {
    if let Some(can_contain) = bags.get(color) {
        let mut num_bags = 0;
        for (inner_bag, inner_amount) in can_contain.iter() {
            num_bags += inner_amount + (inner_amount * find_total_bags_containing(inner_bag, bags));
        }
        num_bags
    } else {
        0
    }
}

fn contains_nested(
    color: &str,
    bags: &HashMap<&str, HashMap<&str, u32>>,
    looking_for: &str,
) -> bool {
    if let Some(can_contain) = bags.get(color) {
        if can_contain.is_empty() {
            false
        } else if can_contain.contains_key(looking_for) {
            true
        } else {
            for bag in can_contain.keys() {
                if contains_nested(bag, bags, looking_for) {
                    return true;
                }
            }
            false
        }
    } else {
        false
    }
}

fn parse(input: &str) -> std::result::Result<HashMap<&str, HashMap<&str, u32>>, &'static str> {
    fn parse_color(input: &str) -> IResult<&str, &str> {
        terminated(take_until(" bag"), alt((tag(" bags"), tag(" bag"))))(input)
    }

    fn parse_bag_and_amount(input: &str) -> IResult<&str, (u32, &str)> {
        separated_pair(
            map(take_while(|chr: char| chr.is_ascii_digit()), |num: &str| {
                num.parse::<u32>().unwrap()
            }),
            tag(" "),
            parse_color,
        )(input)
    }

    fn parse_contained_bags(input: &str) -> IResult<&str, HashMap<&str, u32>> {
        alt((
            map(terminated(tag("no other bags."), newline), |_| {
                HashMap::default()
            }),
            map(
                many_till(
                    terminated(parse_bag_and_amount, alt((tag(", "), tag(".")))),
                    newline,
                ),
                |(a, _)| {
                    a.into_iter()
                        .map(|(a, b)| (b, a))
                        .collect::<HashMap<_, _>>()
                },
            ),
        ))(input)
    }

    fn parse_line(input: &str) -> IResult<&str, (&str, HashMap<&str, u32>)> {
        pair(
            terminated(parse_color, tag(" contain ")),
            parse_contained_bags,
        )(input)
    }

    let (_, (bags, _)) = many_till(parse_line, eof)(input).map_err(|_| "Unable to parse color")?;
    Ok(bags.into_iter().collect::<HashMap<_, _>>())
}
