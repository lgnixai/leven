use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, multispace0};
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use tsr_ast::node::enumeration::EnumDeclaration;
use tsr_ast::node::ident::Ident;
use tsr_lexer::globals::{Input, PineResult, Positioned};
use crate::parsing::parse_ident;
use crate::tags::spaned;


pub fn parse_identifier(input: Input) -> PineResult<Positioned<Ident>> {
    // spaned(map((
    //     tuple(preceded(multispace0, alphanumeric1)
    //     )),|()| Ident::new(name)
    // ))(input)

    spaned(map(
        preceded(multispace0, alphanumeric1),
        |s: Input| Ident::new(*s.fragment()), // 这里将 &str 转换为 String
    ))(input)
}