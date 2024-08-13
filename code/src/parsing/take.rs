// use crate::ast::node::AstNode::Stmt;
// use crate::ast::node::Stmt::VariableDeclaration;
// use crate::input::Input;
// use crate::parsing::parse_assign::parse_assignment;
// use crate::parsing::parse_declaration_mode::parse_declaration_mode;
//
// #[test]
// fn main() {
//    // let a = parse_declaration_mode("varip float speed = 1.5");
//     println!("===={:?}", a);
//     // 测试一些例子
//     let test_cases = vec![
//         "int myVar = 5",
//         "varip float speed = 1.5",
//         "var bool isActive = true",
//         "string name = 6",
//         "var array<int> myArray = array.new(0)",
//     ];
//
//     for test in test_cases {
//         match parse_assignment(Input::new_extra()) {
//             Ok((remaining, var_decl)) => {
//                 println!("Parsed variable declaration: {:?}", var_decl);
//                 println!("Remaining input: {:?}", remaining);
//             }
//             Err(err) => println!("Failed to parse '{}': {:?}", test, err),
//         }
//     }
// }
