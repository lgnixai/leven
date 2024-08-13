use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{alpha1, alphanumeric1, char},
    combinator::{recognize},
    sequence::{pair, preceded},
    IResult,
};
use nom::bytes::complete::take_while;

fn is_valid_start_char(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_valid_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

// 识别标识符的解析器
pub fn parse_identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        take_while(is_valid_start_char),
        take_while(is_valid_char)
    ))(input)
}

#[test]
// 测试解析器
fn main() {
    let inputs = vec![
        " myVar",
        "_myVar",
        " my123Var",
        "functionName",
        "MAX_LEN",
        "max_len",
        "maxLen",
        "3barsDown",
        "InvalidIdentifier!",
    ];

    for input in inputs {
        match parse_identifier(input) {
            Ok((remaining, parsed)) => {
                println!("Parsed identifier: {}, Remaining: {}", parsed, remaining);
            }
            Err(err) => {
                println!("Failed to parse '{}': {:?}", input, err);
            }
        }
    }
}
