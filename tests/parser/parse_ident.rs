use nom::bytes::complete::take;
use nom::IResult;
fn parser(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take(3usize)(input)
}
#[cfg(test)]
mod tests {
    use std::fs;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alphanumeric1, multispace0};
    use nom::combinator::map;
    use nom::{IResult, Parser};
    use nom::sequence::preceded;
    use nom_locate::LocatedSpan;
    use tsr_lexer::globals::{new_input, Positioned, Span};
    use tsr_lexer::{lex_comment, lex_tokens, Lexer};
     use tsr_lexer::tokens::Tokens;

    use tsr_parser::parser::help::tags::{const_tag, ok_tag};
    use tsr_parser::parsing::statement::expression::parse_expression;
    use tsr_parser::tags::{fat_arrow_tag, position};

    pub type Input<'a> = LocatedSpan<&'a str>;

    struct Ident<'a> {
        name: &'a str,
    }

    impl<'a> Ident<'a> {
        fn new(name: &'a str) -> Self {
            Ident { name }
        }
    }

    fn parse_identifier(input: Input) -> nom::IResult<Input, Ident> {
        map(
            preceded(multispace0, alphanumeric1),
            |s: Input| Ident::new(*s.fragment()),
        )(input)
    }
    #[test]
    fn test_tag(){
       // tag("ok")
       fn parser1(s: &str) -> IResult<&str, &str> {
           tag("ok")(s)
       }
        let input = Input::new("const ");
        let b=const_tag(input);
        println!("=={:?}",b);

    }

    fn test_arrow(){

        // let input = Input::new(" myVar");
        //
        // let parser=Parser::new();
        //
        // let result = parse_identifier(input);
        //
        // match result {
        //     Ok((remaining, ident)) => {
        //         println!("Parsed identifier: {:?}", ident.name);
        //         println!("Remaining: {:?}", remaining.fragment());
        //     },
        //     Err(e) => println!("Error: {:?}", e),
        // }
        // let input = b"hello";
        // let result = parser(input);
        // // 处理结果
        // match result {
        //     Ok((remaining, parsed)) => {
        //         let parsed_str = String::from_utf8_lossy(parsed).to_string();
        //         let remaining_str = String::from_utf8_lossy(remaining).to_string();
        //         println!("Parsed: {}", parsed_str);
        //         println!("Remaining: {}", remaining_str);
        //     }
        //     Err(e) => {
        //         println!("Error: {:?}", e);
        //     }
        // }
        // // println!("lex_comment{:?}",d);
        // let a=Positioned::new(Ident("console.log".to_string()), Span::default());
        // let binding=[a];
        // let token=Tokens::new(&binding);
        // //
        //  println!("token{:?},postion{:?}",token,position(token));
        //
        // assert_eq!(2 + 2, 4);
    }

}