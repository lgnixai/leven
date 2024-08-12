use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::Parser as _;
use tsr_ast::node::statement::Statement;

use tsr_lexer::globals::{Input, PineResult, Positioned};
use tsr_lexer::globals::TokenResult;
use tsr_lexer::state::AstState;
use tsr_lexer::token::ReservedWord::As;

use tsr_lexer::token::Token;
use tsr_lexer::tokens::Tokens;
use crate::parsing::statement::parse_program_statement;
use crate::tags::positioned;

use self::ast::Block;
use self::tags::eof_tag;
use self::tags::position;

pub mod ast;
pub mod parsing;
pub mod tags;
pub mod parser;

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
    pub fn parse_ast<'a>(&'a mut self, input: Input<'a>) -> PineResult<Positioned<Statement>>{


        let a=self.parse_program_statement(input);
        return a;
        // map(
        //     tuple((position, many0(parse_program_statement), eof_tag)),
        //     |(start, program, end)| start.between(end.span).wrap(program),
        // )
        //     .parse(Tokens::new(tokens))
    }
    // pub fn parse_tokens<'a>(&'a mut self,tokens: &'a [Positioned<Token>]) -> TokenResult<Block> {
    //     // let parse_program_statement = |input| self.parse_program_statement(input);
    //     //
    //     // map(
    //     //     tuple((position, many0(parse_program_statement), eof_tag)),
    //     //     |(start, program, end)| start.between(end.span).wrap(program),
    //     // )
    //     // .parse(Tokens::new(tokens))
    //     map(
    //         tuple((position, many0(positioned(parse_program_statement)), eof_tag)),
    //         |(start, program, end)| start.between(end.span).wrap(program),
    //     )
    //     .parse(Tokens::new(tokens))
    // }

    pub fn parse_tokens(tokens: &[Positioned<Token>]) -> TokenResult<Block> {
        map(
            tuple((position, many0(parse_program_statement), eof_tag)),
            |(start, program, end)| start.between(end.span).wrap(program),
        )
            .parse(Tokens::new(tokens))
    }
}
