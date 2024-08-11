use nom::bytes::complete::take;
use nom::IResult;
fn parser(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take(3usize)(input)
}
#[cfg(test)]
mod tests {
    use std::fs;
    use nom::sequence::preceded;
    use tsr_lexer::globals::{Positioned, Span};
    use tsr_lexer::{lex_comment, lex_tokens, Lexer};
    use tsr_lexer::token::Token::Ident;
    use tsr_lexer::tokens::Tokens;
    use tsr_parser::Parser;
    use tsr_parser::parsing::statement::expression::parse_expression;
    use tsr_parser::tags::{fat_arrow_tag, position};
    use crate::parse_arrow::parser;

    #[test]
    fn test_arrow(){
        let input = b"hello";
        let result = parser(input);
        // 处理结果
        match result {
            Ok((remaining, parsed)) => {
                let parsed_str = String::from_utf8_lossy(parsed).to_string();
                let remaining_str = String::from_utf8_lossy(remaining).to_string();
                println!("Parsed: {}", parsed_str);
                println!("Remaining: {}", remaining_str);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
        // // println!("lex_comment{:?}",d);
        // let a=Positioned::new(Ident("console.log".to_string()), Span::default());
        // let binding=[a];
        // let token=Tokens::new(&binding);
        // //
        //  println!("token{:?},postion{:?}",token,position(token));
        //
        // assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_lexer() {
        let input = fs::read_to_string("/bot/leven/tests/parser/main.tsx").unwrap();

        println!("{:?}",input);
        let code = input.as_bytes();

        let (_, tokens) = Lexer::lex_tokens(code.into()).unwrap();
        let (_, ast) = Parser::parse_tokens(&tokens).unwrap();


        //let c=preceded(fat_arrow_tag, parse_expression)(ast);
        println!("{:?}",ast);

    }
}