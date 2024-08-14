use std::path::PathBuf;
use crate::ast::node::AstNode::Stmt;
use crate::input::Input;
use crate::inputctx::ParserCtx;
use crate::parsing::parse_assign::parse_assignment;
use crate::parsing::parse_declaration_mode::parse_declaration_mode;
use crate::parsing::parse_identifier::parse_identifier;

#[test]
fn main() {
   // let a = parse_declaration_mode("varip float speed = 1.5");
    //println!("===={:?}", a);
    // 测试一些例子
    let test_cases = vec![
        "int myVar = 5",
        "varip float speed = 1.5",
        "var bool isActive = true",
        "string name = \"adfasdf\"",
        "var array<int> myArray = array.new(0)",
    ];

    for test in test_cases {
        let mut path = PathBuf::new();
        let ctx=ParserCtx::new(path);
        match parse_assignment(Input::new_extra(test,ctx)) {
       // match parse_assignment(Input::new_extra()) {
            Ok((remaining, var_decl)) => {
                println!("Parsed variable declaration: {:?}", var_decl);
                println!("Remaining input: {:?}", remaining);
            }
            Err(err) => println!("Failed to parse '{}': {:?}", test, err),
        }
    }
}
