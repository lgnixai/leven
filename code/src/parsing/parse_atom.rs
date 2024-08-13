use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, i64};
use nom::combinator::map;
use nom::IResult;
use nom::number::complete::double;
use nom::sequence::delimited;
use crate::ast::node::{Atom, Expr};
use crate::input::{Input, PineResult};

fn parse_string(input: Input) -> PineResult< Atom> {
    let (input, s) = delimited(tag("\""), take_until("\""), tag("\""))(input)?;
    Ok((input, Atom::String(s.to_string())))
}

fn parse_boolean(input: Input) -> PineResult< Atom> {
    alt((
        map(tag("true"), |_| Atom::Boolean(true)),
        map(tag("false"), |_| Atom::Boolean(false)),
    ))(input)
}

fn parse_double(input: Input) -> PineResult< Atom> {
    map(double, Atom::Double)(input)
}

fn parse_integer(input: Input) -> PineResult< Atom> {
    map(i64, Atom::Integer)(input)
}

fn parse_variable(input: Input) -> PineResult< Atom> {
    map(alpha1, |var:Input| Atom::Variable(var.to_string()))(input)
}

pub fn parse_atom(input: Input) -> PineResult<Expr> {
    let parser = alt((
        parse_boolean,
        parse_double,
        parse_integer,
        parse_string,
        parse_variable,
    ));
    map(parser, |atom| atom.into())(input)
}
