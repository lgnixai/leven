use std::path::PathBuf;
use nom::{IResult, error::{ErrorKind, Error}};
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{line_ending, multispace0, space0};
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, pair, tuple};
use nom_locate::LocatedSpan;
use crate::input::Input;
use crate::inputctx::ParserCtx;
use nom::Parser;

//pub type Input<'a> = LocatedSpan<&'a str>;

type GreedyError<I> = Error<I>;

pub trait MyParser<'a, O>: nom::Parser<Input<'a>, O, GreedyError<Input<'a>>> {}

impl<'a, F, O> MyParser<'a, O> for F
    where
        F: nom::Parser<Input<'a>, O, GreedyError<Input<'a>>>
{}


pub fn discard_newline_indent(prefix: Option<&str>) -> impl MyParser<()> {
    move |input| {
        if let Some(prefix) = prefix {
            discard(tuple((eol(), blank_lines(), tag(prefix)))).parse(input)
        } else {
            Ok((input, ()))
        }
    }
}

pub fn discard_indent(prefix: Option<&str>) -> impl MyParser<()> {
    move |input| {
        if let Some(prefix) = prefix {
            discard(tag(prefix)).parse(input)
        } else {
            Ok((input, ()))
        }
    }
}

pub fn ws<'a, F, O>(inner: F) -> impl MyParser<'a, O>
    where
        F: MyParser<'a, O>,
{
    delimited(space0, inner, space0)
}

pub fn multiline_ws<'a, F, O>(inner: F) -> impl MyParser<'a, O>
    where
        F: MyParser<'a, O>,
{
    delimited(multispace0, inner, multispace0)
}
// //
pub fn discard<'a, F, O>(inner: F) -> impl MyParser<'a, ()>
    where
        F: MyParser<'a, O>,
{
    map(inner, |_| ())
}
// //
// //
pub fn blank_lines<'a>() -> impl MyParser<'a, ()> {
    discard(many0(pair(space0, eol())))
}

pub fn eol<'a>() -> impl MyParser<'a, ()> {
    discard(tuple((
        space0,
        opt(pair(tag("#"), is_not("\r\n"))),
        line_ending,
    )))
}
//type ParseResult<'a, T> = IResult<Input<'a>, T, nom_greedyerror::GreedyError<Input<'a>, ErrorKind>>;

// macro_rules! operators {
//     ($(($name:ident, $op:expr)),*) => {
//         $(
//             fn $name(input: Input) -> ParseResult<()> {
//                 ws(discard(tag($op))).parse(input)
//             }
//         )*
//     }
// }
//
// operators!((colon, ":"));

// let (input, prefix) = preceded(
// pair(eol(), blank_lines()),
// recognize(pair(discard_indent(current_indent), space1)),
// )
#[test]
fn test_eol() {
    let mut parser = blank_lines();
    let mut path = PathBuf::new();
    let ctx=ParserCtx::new(path);

        let input = r#"genv(x,y)=>asdfas+x"#;

println!("{:?}",input);
    let result = parser.parse(Input::new_extra(input,ctx));
    match result {
        Ok((remaining, matched)) => {
            println!("{:?}",remaining.fragment());
            //println!("{:?}",matched);


            // assert_eq!(remaining.fragment(), "");
            // assert_eq!(matched, ());
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
}


