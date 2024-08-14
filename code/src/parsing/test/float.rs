use std::path::PathBuf;
use crate::input::Input;
use crate::inputctx::ParserCtx;
use crate::parsing::utils::blank_lines;

//这里是为什么
use nom::Parser;
#[test]
fn test_eol() {
    let mut pp = blank_lines();
    let mut path = PathBuf::new();
    let ctx=ParserCtx::new(path);

    let input = "genv(x,y)=>asdfas+x\n";

    println!("{:?}",input);
    let result = pp.parse(Input::new_extra(input,ctx));
    match result {
        Ok((remaining, matched)) => {
            println!("{:?}",remaining.fragment());
            //println!("{:?}",matched);


            // assert_eq!(remaining.fragment(), "");
            // assert_eq!(matched, ());
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
}

