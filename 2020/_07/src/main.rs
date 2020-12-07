use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::newline;
use nom::combinator::{eof, map};
use nom::multi::many_till;
use nom::sequence::{pair, separated_pair, terminated};
use nom::IResult;
use std::collections::HashMap;
use std::io::Read;
use std::rc::Rc;
use util::Result;

struct Bag<'a> {
    color: &'a str,
    can_hold: HashMap<&'a str, u8>,
}

fn main() -> Result<()> {
    let mut buf = String::new();
    let mut file = std::fs::File::open("_07/input.txt")?;
    file.read_to_string(&mut buf)?;
    let items = parse(&buf)?;
    println!("{:?}", items.keys().collect::<Vec<_>>());

    // items.iter().filter(|(bag, can_hold)| )
    Ok(())
}

fn parse(input: &str) -> std::result::Result<HashMap<&str, Vec<(u8, &str)>>, &'static str> {
    fn parse_color(input: &str) -> IResult<&str, &str> {
        terminated(take_until(" bag"), alt((tag(" bags"), tag(" bag"))))(input)
    }

    fn parse_bag_and_amount(input: &str) -> IResult<&str, (u8, &str)> {
        separated_pair(
            map(take_while(|chr: char| chr.is_ascii_digit()), |num: &str| {
                num.parse::<u8>()
                    .map_err(|e| {
                        println!("{}", input);
                        e
                    })
                    .unwrap()
            }),
            tag(" "),
            parse_color,
        )(input)
    }

    fn parse_contained_bags(input: &str) -> IResult<&str, Vec<(u8, &str)>> {
        let (input, (contains, _)) = alt((
            map(terminated(tag("no other bags."), newline), |_| {
                (Vec::new(), '_')
            }),
            many_till(
                terminated(parse_bag_and_amount, alt((tag(", "), tag(".")))),
                newline,
            ),
        ))(input)?;

        Ok((input, contains))
    }

    fn parse_line(input: &str) -> IResult<&str, (&str, Vec<(u8, &str)>)> {
        pair(
            terminated(parse_color, tag(" contain ")),
            parse_contained_bags,
        )(input)
    }

    let (_, (bags, _)) = many_till(parse_line, eof)(input).map_err(|_| "Unable to parse color")?;
    // let mut map = HashMap::<&str, Bag>::default();
    //
    // bags.into_iter().for_each(|(color, contains)| {
    //    let bag = map.entry(color) .or_insert_with(|| Bag { color, can_hold: HashMap::new()});
    //     for inner_bag = contains.into_iter().for_each(|(can_hold)| )
    // });
    //
    // unimplemented!()
    Ok(bags.into_iter().collect::<HashMap<_, _>>())
}
