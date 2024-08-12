use tsr_lexer::globals::Positioned;
use crate::node::enumeration::EnumDeclaration;
use crate::node::expression::Expression;
use crate::node::variable_statement::VariableStatement;

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    // ImportDeclaration(Box<Positioned<ImportDeclaration>>),
    // TypeAliasDeclaration(Positioned<TypeAliasDeclaration>),
    // InterfaceDeclaration(Positioned<InterfaceDeclaration>),
    // FunctionDeclaration(Positioned<FunctionDeclaration>),
    EnumDeclaration(Positioned<EnumDeclaration>),
    // ExportDeclaration(Positioned<ExportDeclaration>),
    // ClassDeclaration(Positioned<ClassDeclaration>),
    VariableStatement(Positioned<VariableStatement>),
    // IfStatement(Box<Positioned<IfStatement>>),
    // ReturnStatement(Positioned<Expression>),
    Expression(Positioned<Expression>),
}