use tsr_lexer::globals::Positioned;
use crate::node::statement::Statement;

pub type Block = Positioned<Vec<Positioned<Statement>>>;
