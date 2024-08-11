use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::sequence::terminated;
use tsr_lexer::globals::{Positioned, TokenResult};
use tsr_lexer::state::AstState;
use tsr_lexer::tokens::Tokens;
use crate::ast::Statement;
use crate::Parser;

use crate::parsing::statement::{class, enumeration, export, expression, function, if_else, import, interface, returning, type_alias, variable};
use crate::tags::{positioned, semi_tag};

impl Parser {
    //pub fn parse_program_statement(&self,input: Tokens) -> TokenResult<Positioned<Statement>> {
    pub fn parse_program_statement<'a>(&'a self, input: Tokens<'a>) -> TokenResult<Positioned<Statement>> {
        let parse_function_declaration = |input| self.parse_function_declaration(input);

        terminated(
            positioned(alt((
                map(import::parse_import_declaration, |declaration| {
                    Statement::ImportDeclaration(Box::new(declaration))
                }),
                map(
                    export::parse_export_declaration,
                    Statement::ExportDeclaration,
                ),
                map(
                    type_alias::parse_type_alias_declaration,
                    Statement::TypeAliasDeclaration,
                ),
                map(class::parse_class_declaration, Statement::ClassDeclaration),
                map(
                    interface::parse_interface_declaration,
                    Statement::InterfaceDeclaration,
                ),
                map(
                    parse_function_declaration,
                    Statement::FunctionDeclaration,
                ),
                map(
                    enumeration::parse_enum_declaration,
                    Statement::EnumDeclaration,
                ),
                map(
                    variable::parse_variable_statement,
                    Statement::VariableStatement,
                ),
                map(if_else::parse_if_statement, |statement| {
                    Statement::IfStatement(Box::new(statement))
                }),
                returning::parse_return_statement,
                map(expression::parse_expression, Statement::Expression),
            ))),
            opt(semi_tag),
        )(input)
    }
}