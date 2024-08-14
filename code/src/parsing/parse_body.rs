use std::path::PathBuf;
use nom::character::complete::{line_ending, space1};
use nom::combinator::{map, opt};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::preceded;
use crate::ast::node::Body;
use crate::ast::node::Type::String;
use crate::input::{Input, PineResult};
use crate::inputctx::ParserCtx;
use crate::parsing::parse_enums::parse_enum_declaration;
use crate::parsing::parse_function::parse_function;
use crate::parsing::parse_stmt::parse_stmt;

fn indent<'a, O, E, F>(mut parser: F) -> impl FnMut(Input<'a>) -> IResult<Input<'a>, O, E>
    where
        F: nom::Parser<Input<'a>, O, E>,
{
    move |mut input: Input<'a>| {
        if let Some(indent) = input.extra.first_indent {
            input.extra.block_indent += indent;
        }

        let (mut input, output) = parser.parse(input)?;

        if let Some(indent) = input.extra.first_indent {
            input.extra.block_indent -= indent;
        }

        Ok((input, output))
    }
}

pub fn parse_block_indent(input: Input) -> PineResult<usize> {
    let (mut input, indent) = space1(input)?;
    let indent_len = indent.fragment().len();

    if input.extra.first_indent == None {
        input.extra.first_indent = Some(indent_len);
        input.extra.block_indent = indent_len;
    }

    if indent_len == input.extra.block_indent {
        Ok((input, indent_len))
    } else {
        // todo()
        Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Float)))

        // Err(nom::Err::Error(ParseError::from_error_kind(
        //     input,
        //     ErrorKind::IsA,
        // )))
    }
}

pub fn parse_block_indent_plus_one(input: Input) -> PineResult<usize> {
    let (mut input, indent) = space1(input)?;
    let indent_len = indent.fragment().len();

    if input.extra.first_indent == None {
        input.extra.first_indent = Some(indent_len);
        input.extra.block_indent = indent_len;
    }

    if indent_len == input.extra.block_indent + input.extra.first_indent.unwrap() {
        Ok((input, indent_len))
    } else {
        // todo()
        Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Float)))

        // Err(nom::Err::Error(ParseError::from_error_kind(
        //     input,
        //     ErrorKind::Tag,
        // )))
    }
}

pub fn parse_body(input: Input) -> PineResult<Body> {

    println!("=========fffff");
    let (input, opt_eol) = opt(many1(line_ending))(input)?; // NOTE: should not fail
    //
    if opt_eol.is_some() {
        indent(map(
            separated_list1(
                many1(line_ending),
                preceded(parse_block_indent, parse_stmt),
            ),
            Body::new,
        ))(input)
    } else {
        // let b=String::from(input.fragment().to_string());
        map(parse_stmt, |stmt| Body::new(vec![stmt]))(input)
    }
}

#[test]
fn main() {
    let input = r#"genv(x,y)=>x+3
    "#;
    let mut path = PathBuf::new();
    let ctx=ParserCtx::new(path);

    let result = parse_function(Input::new_extra(input,ctx));
    match result {
        Ok((remaining, enum_decl)) => {
            println!("Parsed enum: {:?}, Remaining input: {}", enum_decl, remaining);
        }
        Err(e) => println!("Error parsing enum: {:?}", e),
    }
}
