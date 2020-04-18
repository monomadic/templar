// https://github.com/Geal/nom/blob/master/doc/choosing_a_combinator.md
// https://github.com/benkay86/nom-tutorial

use crate::*;
use error::*;

use nom::*;
use nom::branch::alt;
use nom::combinator::{ map, opt, value };
use nom::character::complete::{ space0, space1, multispace0, multispace1, alphanumeric1, one_of, char, digit1 };
use nom::number::complete::{ double };
use nom::bytes::complete::{ tag, is_not };
use nom::sequence::preceded;
use nom::sequence::delimited;

/// returns the position of the first non-whitespace character, or None if the line is entirely whitespace.
fn indentation_level(i: &str) -> IResult<&str, usize> {
    nom::multi::many0_count(one_of(" \t"))(i)
}

/// returns a nom combinator version of the parser
pub fn run(i:&str) -> IResult<&str, Vec<Node>> {
    nom::multi::many0(node)(i)
}

fn _node(i: &str) -> IResult<&str, Node> {
    println!("-- {:?}", i);
    alt((
        map(assignment, |node| node),
        overlay_declaration,
        map(block, |(_, ident, _, params, _)| Node::Block {
            ident: String::from(ident),
            attributes: params,
            children: Vec::new(),
        }),
        map(anonymous_property, |p| Node::AnonymousProperty(p)),
        //map(multispace1, |_| Node::WhiteSpace), // remove this
    ))(i)
}

fn anonymous_property(i: &str) -> IResult<&str, Property> {
    // let params = nom::multi::many0(block_property);

    let (remainder, (_, property, _)) = nom::sequence::tuple(
        (multispace0, block_property, take_while_newline)
    )(i)?;

    Ok((remainder, property))
}

fn node(i: &str) -> IResult<&str, Node> {
    let (_, indentation) = indentation_level(i)?;
    let (r, n) =_node(i)?;
    let (_, next_line_indentation) = indentation_level(r)?;
    let mut next_line_indentation = next_line_indentation;
    let mut children = Vec::new();
    let mut remainder = r;

    // if the next line if further indented (a child node of this node),
    while next_line_indentation > indentation {
        // take a node
        let (r, child) = node(remainder)?;
        remainder = r;
        // println!("child found: {:?}", &child);
        children.push(child);

        let (_, next_line) = indentation_level(r)?;
        next_line_indentation = next_line;
    }

    // if the current node is a block, return it
    // ( children are not picked up in the first pass )
    if let Node::Block{ident, attributes, ..} = n {
        return Ok((remainder, Node::Block {
            ident, attributes, children
        }))
    }

    if let Node::Overlay(overlay) = n {
        return Ok((remainder, Node::Overlay(
            Overlay {
                ident: overlay.ident, output: overlay.output, arguments: overlay.arguments, children
            }
        ) ))
    }

    // the node must be a property
    Ok((remainder, n))
}

fn overlay_declaration(i: &str) -> IResult<&str, Node> {
    let (input, (_, ident, _, output, _, arguments, _)) =
        nom::sequence::tuple(
            (multispace0, colon_symbol, space1, symbol, space0, nom::multi::many0(symbol), take_while_newline)
        )(i)?;

    println!("input: {:?}", input);

    return Ok((input,
        Node::Overlay(
            Overlay {
                ident: ident.into(),
                output: output.into(),
                arguments: arguments.into_iter().map(|a| a.to_string()).collect(),
                children: Vec::new(),
            }
        )
    ))
}

/// valid characters for an ident
pub fn symbolic1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: InputTakeAtPosition,
  <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
  input.split_at_position1_complete(|item| {
    let c = item.clone().as_char();
    !(c == '-' || c == '_' || item.is_alphanum())
  },
    ErrorKind::AlphaNumeric
  )
}

pub fn block(i: &str) -> IResult<&str, (&str, &str, &str, Vec<Property>, &str)> {
    let params = nom::multi::many0(block_property);

    nom::sequence::tuple(
        (multispace0, alphanumeric1, space0, params, take_while_newline)
    )(i)
}

fn take_while_newline(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::take_while(|c| c == '\n')(i)
}

fn assignment(i: &str) -> IResult<&str, Node> {
    let space = nom::bytes::complete::take_while(|c| c == ' ');
    let (input, (_, ident, _, value, _)) =
        nom::sequence::tuple(
            (multispace0, dotted_symbol, space, block_property, take_while_newline)
        )(i)?;

    Ok((input, Node::Assignment {
        ident: String::from(ident),
        value
    }))
}

// matches dotted symbols eg .blah .class
fn dotted_symbol(i: &str) -> IResult<&str, &str> {
    trim_pre_whitespace(nom::sequence::preceded(char('.'), symbolic1))(i)
}

// matches overlay declaration eg :blah
fn colon_symbol(i: &str) -> IResult<&str, &str> {
    trim_pre_whitespace(nom::sequence::preceded(char(':'), symbolic1))(i)
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
        map(quoted_string,  |s| Property::QuotedString(String::from(s))),
        map(argument_idx,   |i| Property::ArgumentIndex(i.parse::<usize>().unwrap())),
        map(double,         |f| Property::Float(f)),
        map(digit1,         |i:&str| Property::Number(i.parse::<i64>().unwrap_or(0))),
        map(boolean,        |b| Property::Boolean(b)),
        map(dotted_symbol,  |s| Property::DottedSymbol(String::from(s))),
        map(symbol,         |s| Property::Symbol(String::from(s))),
    ))(i)
}

fn argument_idx(i: &str) -> IResult<&str, &str> {
    trim_pre_whitespace(preceded(char('$'), digit1))(i)
}

fn _argument_idx(i: &str) -> IResult<&str, usize> {
    let (input, (_, _, index, _)) = nom::sequence::tuple(
        (multispace0, char('$'), digit1, take_while_newline)
    )(i)?;

    let index = index.parse::<usize>().unwrap(); // todo: fix this unwrap with an error

    Ok((input, index))
}

/// match an alphanumeric word (symbol) with optional preceding space
fn symbol(i: &str) -> IResult<&str, &str> {
    trim_pre_whitespace(alphanumeric1)(i)
}

fn trim_pre_whitespace<'a, O1, F>(inner: F) -> impl Fn(&'a str) -> IResult<&'a str, O1, (&str, nom::error::ErrorKind)>
where F: Fn(&'a str) -> IResult<&'a str, O1, (&str, nom::error::ErrorKind)>,
{

    preceded(opt(one_of(" \t\n\r")), inner)
}

fn quoted_string(i: &str) -> IResult<&str, &str> {
    trim_pre_whitespace(delimited(
        char('\"'), is_not("\""), char('\"')
    ))(i)
}
