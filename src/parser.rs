use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::IResult;
use nom::multi::separated_list1;
use crate::{Nanobot, Point};

pub fn parse_input(input: &str) -> IResult<&str, Vec<Nanobot>> {
    separated_list1(line_ending, parse_nanobot)(input)
}

fn parse_nanobot(input: &str) -> IResult<&str, Nanobot> {
    let (input, position) = parse_point(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, radius) = parse_radius(input)?;
    Ok((input, Nanobot { position, radius }))
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, _) = tag("pos=<")(input)?;
    let (input, x) = parse_isize(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = parse_isize(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, z) = parse_isize(input)?;
    let (input, _) = tag(">")(input)?;
    Ok((input, (x, y, z)))
}

fn parse_radius(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("r=")(input)?;
    let (input, radius) = parse_usize(input)?;
    Ok((input, radius))
}

fn parse_isize(input: &str) -> IResult<&str, isize> {
    let (input, sign) = alt((tag("+"), tag("-"), tag("")))(input)?;
    let (input, number) = parse_usize(input)?;
    let number = if sign == "-" { -(number as isize) } else { number as isize };
    Ok((input, number))
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    let (input, number) = digit1(input)?;
    let number = number.parse::<usize>().unwrap();
    Ok((input, number))
}