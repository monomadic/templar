// https://github.com/Geal/nom/blob/master/doc/choosing_a_combinator.md
// https://github.com/benkay86/nom-tutorial

use crate::*;
use nom::*;

use nom::branch::alt;
use nom::combinator::{ map, opt, value };
use nom::character::complete::{ space0, multispace1, line_ending, alphanumeric1, one_of, char, digit1 };
use nom::number::complete::{ double };
use nom::bytes::complete::tag;

/// returns the position of the first non-whitespace character, or None if the line is entirely whitespace.
fn indentation_level(i: &str) -> IResult<&str, usize> {
    nom::multi::many0_count(one_of(" \t"))(i)
}

/// parses a string into a node graph
pub fn parse(i:&str) -> IResult<&str, Vec<Node>> {
    nom::multi::many0(node)(i)
}

fn _node(i: &str) -> IResult<&str, Node> {
    alt((
        map(block, |(_, ident, _, params, _)| Node::Block {
            ident: String::from(ident),
            properties: params,
            children: Vec::new(),
        }),
        map(multispace1, |_| Node::WhiteSpace),
    ))(i)
}

fn node(i: &str) -> IResult<&str, Node> {
    let (_, indentation) = indentation_level(i)?;
    let (mut r, mut n) =_node(i)?;
    let (_, next_line_indentation) = indentation_level(r)?;

    if next_line_indentation > indentation {
        let (r, children) = nom::multi::many0(node)(r)?;
        match n {
            Node::Block{ident, properties, ..} => return Ok((r, Node::Block {
                ident, properties, children
            })),
            _ => (),
        }
        println!("children: {:?}", children);
    }

    Ok((r, n))
}

pub fn block(i: &str) -> IResult<&str, (&str, &str, &str, Vec<Property>, &str)> {
    let params = nom::multi::many0(block_property);
    nom::sequence::tuple(
        (space0, alphanumeric1, space0, params, line_ending)
    )(i)
}

fn node_assignment(i: &str) -> IResult<&str, Node> {
    let space = nom::bytes::complete::take_while(|c| c == ' ');
    let (input, (ident, _, value)) =
        nom::sequence::tuple((dotted_symbol, space, block_property))(i)?;

    Ok((input, Node::Assignment {
        ident: String::from(ident),
        value
    }))
}

// matches dotted symbols eg .blah .class
fn dotted_symbol(i: &str) -> IResult<&str, &str> {
    nom::sequence::preceded(char('.'), alphanumeric1)(i)
}

// matches function calls inside nodes eg <fn_name> <args>
fn node_call(i: &str) -> IResult<&str, Node> {
    let space = nom::bytes::complete::take_while(|c| c == ' ');
    let params = nom::multi::many0(block_property);
    let (input, (ident, _, properties)) =
        nom::sequence::tuple((symbol, space, params))(i)?;

    Ok((input, Node::Block {
        ident: String::from(ident),
        properties: Vec::new(),
        children: Vec::new(),
    }))
}

fn parse_block_header(i: &str) -> IResult<&str, Block> {
    let space = nom::bytes::complete::take_while(|c| c == ' ');
    let method = nom::bytes::complete::take_while1(nom::AsChar::is_alpha);
    let params = nom::multi::many0(block_property);

    let (r, (ident, _, properties)) =
        nom::sequence::tuple((method, space, params))(i)?;

    Ok((r, Block {
        ident: String::from(ident),
        properties,
        nodes: Vec::new(),
    }))
}

fn boolean(i: &str) -> IResult<&str, bool> {
    alt((
        value(true, tag("true")),
        value(false, tag("false")),
    ))(i)
}

// return custom enum later
fn block_property(i: &str) -> IResult<&str, Property> {
    alt((
        // map(hash, JsonValue::Object),
        // map(array, JsonValue::Array),
        map(quoted_string, |s| Property::QuotedString(String::from(s))),
        map(double, |f| Property::Float(f)),
        map(digit1, |i:&str| Property::Number(i.parse::<i64>().unwrap_or(0))),
        map(boolean, |b| Property::Boolean(b)),
        map(symbol, |s| Property::Symbol(String::from(s))),
    ))(i)
}

fn parse_str(input: &str) -> IResult<&str, &str> {
    use nom::character::complete::alphanumeric0;

    alphanumeric0(input)
}

/// match an alphanumeric word (symbol) with optional preceding space
fn symbol(i: &str) -> IResult<&str, &str> {
    trim_pre_whitespace(alphanumeric1)(i)
}

fn trim_pre_whitespace<'a, O1, F>(inner: F) -> impl Fn(&'a str) -> IResult<&'a str, O1, (&str, nom::error::ErrorKind)>
where
  F: Fn(&'a str) -> IResult<&'a str, O1, (&str, nom::error::ErrorKind)>,
{
    use nom::sequence::preceded;

    preceded(opt(one_of(" \t")), inner)
}

fn quoted_string(i: &str) -> IResult<&str, &str> {
    use nom::bytes::complete::is_not;
    use nom::sequence::delimited;

    trim_pre_whitespace(delimited(
        char('\"'), is_not("\""), char('\"')
    ))(i)
}
