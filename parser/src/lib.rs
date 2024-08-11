use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::Parser as _;

use tsr_lexer::globals::Positioned;
use tsr_lexer::globals::TokenResult;
use tsr_lexer::state::AstState;
use tsr_lexer::token::ReservedWord::As;

use tsr_lexer::token::Token;
use tsr_lexer::tokens::Tokens;
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
    pub fn parse_tokens<'a>(&'a mut self,tokens: &'a [Positioned<Token>]) -> TokenResult<Block> {
        let parse_program_statement = |input| self.parse_program_statement(input);

        map(
            tuple((position, many0(parse_program_statement), eof_tag)),
            |(start, program, end)| start.between(end.span).wrap(program),
        )
        .parse(Tokens::new(tokens))
        // map(
        //     tuple((position, many0(positioned(self.parse_program_statement)), eof_tag)),
        //     |(start, program, end)| start.between(end.span).wrap(program),
        // )
        // .parse(Tokens::new(tokens))
    }
}
