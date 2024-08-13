use nom::combinator::map;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::tuple;
use crate::ast::state::AstState;

pub mod parsing;
pub mod ast;
pub mod parser;
pub mod input;
pub mod tags;
mod err;
mod inputctx;

pub struct Parser{
    pub state:AstState,

}

impl Parser {
    pub fn new() -> Self {
        let state=AstState::new();
        Parser {
            state
        }
    }

    //解析成语法树
    // pub fn parse_ast<'a>(&'a mut self, input: &str<'a>) -> IResult<&str>{
    //
    //
    //     let a=self.parse_program_statement(input);
    //     return a;
    //     // map(
    //     //     tuple((position, many0(parse_program_statement), eof_tag)),
    //     //     |(start, program, end)| start.between(end.span).wrap(program),
    //     // )
    //     //     .parse(Tokens::new(tokens))
    // }



}
