use std::path::PathBuf;
use nom::{
    IResult,
    bytes::complete::{tag, take_while1, is_not},
    character::complete::{multispace0, multispace1, char, line_ending},
    sequence::{preceded, tuple},
    combinator::{map, opt},
    multi::{many1, separated_list1},
};
use crate::ast::enums::{EnumDeclaration, EnumField};
use crate::input::{Input, PineResult};
use crate::inputctx::ParserCtx;
use crate::parsing::parse_identifier::parse_identifier;


// 解析标识符（例如 Signal、buy、sell）
// fn parse_identifier(input: &str) -> IResult<&str, String> {
//     map(take_while1(|c: char| c.is_alphanumeric() || c == '_'), |s: &str| s.to_string())(input)
// }

// 解析枚举字段及其可选标题
// fn parse_enum_field(input: Input) -> PineResult<  EnumField> {
//
//     map(tuple((
//         parse_identifier,
//         opt(preceded(
//             tuple((multispace0, char('='), multispace0)),
//             // 解析字段标题，允许引号
//             map(is_not("\n"), |s: &str| s.trim_matches('"').to_string()),
//         )),
//     )),|(ident,title)|EnumField { name:ident.name, title })(input)
//
// }

// 解析枚举声明
pub fn parse_enum_declaration(input: Input) -> PineResult<  EnumDeclaration> {
    // 忽略前导空白符并解析 enum 关键字
    let (input, _) = preceded(multispace0, tag("enum"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, ident) = parse_identifier(input)?;
    let (input, _) = line_ending(input)?; // 解析枚举名后的换行符

    // 解析枚举字段，按行分隔
    //let (input, fields) = separated_list1(line_ending, preceded(multispace1, parse_enum_field))(input)?;

    Ok((input, EnumDeclaration { name:ident.name }))
}

#[test]
fn main() {
    let input = r#"
    enum Signal
        buy     = "Buy signal"
        sell    = "Sell signal"
        neutral
    "#;
    let mut path = PathBuf::new();
    let ctx=ParserCtx::new(path);

    let result = parse_enum_declaration(Input::new_extra(input,ctx));
    match result {
        Ok((remaining, enum_decl)) => {
            println!("Parsed enum: {:?}, Remaining input: {}", enum_decl, remaining);
        }
        Err(e) => println!("Error parsing enum: {:?}", e),
    }
}
