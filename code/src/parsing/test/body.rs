// use std::path::PathBuf;
// use crate::input::Input;
// use crate::inputctx::ParserCtx;
// use crate::parsing::utils::blank_lines;
//
// #[test]
// fn test_eol() {
//     let mut parser = blank_lines();
//     let mut path = PathBuf::new();
//     let ctx=ParserCtx::new(path);
//
//     let input = r#"genv(x,y)=>asdfas+x"#;
//
//     println!("{:?}",input);
//     let result = parser.parse(Input::new_extra(input,ctx));
//     match result {
//         Ok((remaining, matched)) => {
//             println!("{:?}",remaining.fragment());
//             //println!("{:?}",matched);
//
//
//             // assert_eq!(remaining.fragment(), "");
//             // assert_eq!(matched, ());
//         }
//         Err(e) => {
//             panic!("Error: {:?}", e);
//         }
//     }
// }