use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::sequence::terminated;
use tsr_lexer::globals::{Input, PineResult, Positioned, TokenResult};
use tsr_lexer::state::AstState;
use tsr_lexer::tokens::Tokens;
 use crate::Parser;
use tsr_ast::node::statement::Statement;

use crate::parsing::statement::{class, enumeration, export, expression, function, if_else, import, interface, returning, type_alias, variable};
use crate::tags::{positioned, semi_tag, spaned};

impl Parser {
    pub fn parse_program_statement<'a>(&'a self, input: Input<'a>) -> PineResult<Positioned<Statement>> {

       let parse_enum_declaration=|input|self.parse_enum_declaration(input);
       let parse_variable_statement=|input|self.parse_variable_statement(input);
        terminated(
            spaned(alt((
                map(
                    parse_enum_declaration,
                    Statement::EnumDeclaration,
                ),
                map(
                    parse_variable_statement,
                    Statement::EnumDeclaration,
                ),
            ))),
            opt(tag(";")),
        )(input)
    }
}