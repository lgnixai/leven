use std::io::ErrorKind;
use nom::bytes::streaming::take_while1;
use nom::character::complete::{alpha1, char};
use nom::character::{is_alphabetic};
use nom::combinator::recognize;
use nom::IResult;
use nom::sequence::separated_pair;

fn alpha(s: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while1(is_alphabetic)(s)
}

#[test]
fn main(){
    assert_eq!(alpha(b"latin123"), Ok((&b"123"[..], &b"latin"[..])));
    let mut parser = recognize(separated_pair(alpha1, char(','), alpha1));

    assert_eq!(parser. parser("abcd,efgh"), Ok(("", "abcd,efgh")));
    //assert_eq!(parser. parse("abcd;"),Err(Err::Error((";", ErrorKind::Char))));

}