// https://github.com/Geal/nom/blob/master/doc/choosing_a_combinator.md
// https://github.com/benkay86/nom-tutorial

use crate::*;
use nom::*;

/// returns the position of the first non-whitespace character, or None if the line is entirely whitespace.
fn indentation_level(str: &str) -> Option<usize> {
    str.chars().position(|c| !c.is_whitespace())
}

/// parses a string into a node graph
pub fn parse(content:&str) -> ParseResult<Vec<Block>> {
    let lines: Vec<String> = content.lines().map(|s|s.to_string()).collect();
    let mut blocks: Vec<Block> = Vec::new();

    for (line_index, line) in lines.iter().enumerate() {
        // look for first non-whitespace char,
        if let Some(indentation_level) = indentation_level(line) {
            let (_, line_without_whitespace) = line.split_at(indentation_level);

            if indentation_level == 0 {
                let (_, block_header) = parse_block_header(line_without_whitespace).unwrap();

                blocks.push(block_header);
            }

        }
    }

    Ok(blocks)
}

// use nom::character::complete::digit1;
// use nom::combinator::map;

// let parse_bh = map(alphanumeric0, |s: &str| s.len());

// let split_by_space = opt()

// let block_header_ident = alphanumeric0;

fn parse_block_header(input: &str) -> IResult<&str, Block> {
    let space = nom::bytes::complete::take_while1(|c| c == ' ');

    // println!("split: {:?}", nom::multi::separated_list(char(' '), alphanumeric0)(input));

    println!("INPUT: {}", input);
    let (properties, ident) = alphanumeric0(input)?;

    let method = nom::bytes::complete::take_while1(nom::AsChar::is_alpha);
    
    let params = nom::multi::many0(block_parameter);
    // nom::multi::separated_list(char(' '), alphanumeric0)
    // let params = nom::bytes::complete::take_while1(nom::AsChar::is_alpha);


    let (input, (ident, _, parameters)) =
        nom::sequence::tuple((method, space, params))(input)?;


    println!("(ident, parameters): {:?}", (ident, parameters));

    Ok(("", Block {
        ident: String::from(ident),
        properties: Vec::new(),
        nodes: Vec::new(),
    }))
}

// fn trim_whitespace(i: &str) -> IResult<&str, String> {
// }

// return custom enum later
fn block_parameter(i: &str) -> IResult<&str, String> {


    // let (_, i) = take_while!(one_of!(" \t"))(i)?;
    // let (a, b) = nom::bytes::complete::take_while(|c| c == ' ')(i)?;
    // let (a, b) = take_while(nom::character::is_space)(i)?;
    // println!("-- {:?}", (a,b));
    // preceded!(opt!(consume_useless_chars),
    // opt(preceded(tag(" "), symbol))
    alt((
        // map(hash, JsonValue::Object),
        // map(array, JsonValue::Array),
        map(quoted_string, |s| String::from(s)),
        // map(double, JsonValue::Num),
        // map(boolean, JsonValue::Boolean),
        map(symbol, |s| String::from(s)),
    ))(i)
}


use nom::combinator::map;
use nom::branch::alt;
use nom::sequence::delimited;
use nom::character::is_alphabetic;
use nom::bytes::complete::take_while;
use nom::character::complete::{
    char, alphanumeric0, alpha1, digit1
};

fn parse_str(input: &str) -> IResult<&str, &str> {
    alphanumeric0(input)
}

/// match an alphanumeric word (symbol) with optional preceding space
fn symbol(i: &str) -> IResult<&str, &str> {
    use nom::combinator::opt;
    use nom::sequence::preceded;
    use nom::complete::tag;
    use nom::character::complete::alphanumeric1;
    use nom::character::complete::one_of;
    use nom::character::is_space;

    trim_pre_whitespace(alphanumeric1)(i)
}

use nom::error::VerboseError;
fn trim_pre_whitespace<'a, O1, F>(inner: F) -> impl Fn(&'a str) -> IResult<&'a str, O1, (&str, nom::error::ErrorKind)>
where
  F: Fn(&'a str) -> IResult<&'a str, O1, (&str, nom::error::ErrorKind)>,
{
    use nom::sequence::preceded;
    use nom::combinator::opt;
    use nom::character::complete::one_of;

    preceded(opt(one_of(" \t")), inner)
}

fn quoted_string(i: &str) -> IResult<&str, &str> {
    use nom::bytes::complete::is_not;

    trim_pre_whitespace(delimited(
        char('\"'), is_not("\""), char('\"')
    ))(i)
}

// named!(consume_useless_chars, take_while!(is_whitespace));
