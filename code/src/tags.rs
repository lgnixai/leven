use nom::{
    bytes::complete::take,
    combinator::{map, verify},
    error::Error,
    sequence::tuple,
    Parser,
};
use nom::bytes::complete::tag;
use crate::input::{Input, PineResult, Positioned, Span};

macro_rules! tags {
    ($($func_name:ident => $tag:expr;)*) => {
        $(
            pub fn $func_name(input: Input<'_>) -> PineResult<Positioned<Input<'_>>> {
                verify(map(tag($tag), |s: Input<'_>| to_positioned(s)), |pos: &Positioned<Input<'_>>| {
                    println!("{:?}", pos.value.fragment());
                    pos.value.fragment().to_string() == $tag
                })(input)

            }
        )*
    };
}

tags! {
    const_tag => "const";
    // let_tag => Token::ReservedWord(ReservedWord::Let);
    // constructor_tag => Token::ReservedWord(ReservedWord::Constructor);
    // class_tag => Token::ReservedWord(ReservedWord::Class);
    // interface_tag => Token::ReservedWord(ReservedWord::Interface);
    // implements_tag => Token::ReservedWord(ReservedWord::Implements);
    // this_tag => Token::ReservedWord(ReservedWord::This);
    // return_tag => Token::ReservedWord(ReservedWord::Return);
    // function_tag => Token::ReservedWord(ReservedWord::Function);
    // if_tag => Token::ReservedWord(ReservedWord::If);
    // else_tag => Token::ReservedWord(ReservedWord::Else);
    // new_tag => Token::ReservedWord(ReservedWord::New);
    // null_tag => Token::ReservedWord(ReservedWord::Null);
    // enum_tag => Token::ReservedWord(ReservedWord::Enum);
    // namespace_tag => Token::ReservedWord(ReservedWord::Namespace);
    // decalre_tag => Token::ReservedWord(ReservedWord::Declare);
    // export_tag => Token::ReservedWord(ReservedWord::Export);
    // import_tag => Token::ReservedWord(ReservedWord::Import);
    // default_tag => Token::ReservedWord(ReservedWord::Default);
    // match_tag => Token::ReservedWord(ReservedWord::Match);
    // extends_tag => Token::ReservedWord(ReservedWord::Extends);
    // get_tag => Token::ReservedWord(ReservedWord::Get);
    // set_tag => Token::ReservedWord(ReservedWord::Set);
    // type_tag => Token::ReservedWord(ReservedWord::Type);
    // typeof_tag => Token::ReservedWord(ReservedWord::TypeOf);
    // for_tag => Token::ReservedWord(ReservedWord::For);
    // in_tag => Token::ReservedWord(ReservedWord::In);
    // of_tag => Token::ReservedWord(ReservedWord::Of);
    // as_tag => Token::ReservedWord(ReservedWord::As);
    // from_tag => Token::ReservedWord(ReservedWord::From);
    // when_tag => Token::ReservedWord(ReservedWord::When);
    //
    // comma_tag => Token::Punctuation(Punctuation::Comma);
    // dot_tag => Token::Punctuation(Punctuation::Dot);
    // ellipsis_tag => Token::Punctuation(Punctuation::Ellipsis);
    // fat_arrow_tag => Token::Punctuation(Punctuation::FatArrow);
    // double_slash_tag => Token::Punctuation(Punctuation::DoubleSlash);
    // double_quote_tag => Token::Punctuation(Punctuation::DoubleQuote);
    // colon_tag => Token::Punctuation(Punctuation::Colon);
    // semi_tag => Token::Punctuation(Punctuation::Semi);
    // question_tag => Token::Punctuation(Punctuation::Question);
    // pound_tag => Token::Punctuation(Punctuation::Pound);
    //
    // and_tag => Token::Operator(Operator::And);
    // and_and_tag => Token::Operator(Operator::AndAnd);
    // plus_tag => Token::Operator(Operator::Plus);
    // star_tag => Token::Operator(Operator::Star);
    // slash_tag => Token::Operator(Operator::Slash);
    // or_tag => Token::Operator(Operator::Or);
    // or_or_tag => Token::Operator(Operator::OrOr);
    // plus_plus_tag => Token::Operator(Operator::PlusPlus);
    // minus_tag => Token::Operator(Operator::Minus);
    // minus_minus_tag => Token::Operator(Operator::MinusMinus);
    // eq_eq_tag => Token::Operator(Operator::EqEq);
    // eq_tag => Token::Operator(Operator::Eq);
    // ne_tag => Token::Operator(Operator::Ne);
    // le_tag => Token::Operator(Operator::Le);
    // ge_tag => Token::Operator(Operator::Ge);
    // lt_tag => Token::Operator(Operator::Lt);
    // gt_tag => Token::Operator(Operator::Gt);
    // not_tag => Token::Operator(Operator::Not);
    //
    // public_tag => Token::Modifier(Modifier::Public);
    // private_tag => Token::Modifier(Modifier::Private);
    // protected_tag => Token::Modifier(Modifier::Protected);
    // static_tag => Token::Modifier(Modifier::Static);
    // async_tag => Token::Modifier(Modifier::Async);
    //
    // any_tag => Token::BuiltInType(BuiltInType::Any);
    // number_tag => Token::BuiltInType(BuiltInType::Number);
    // float_tag => Token::BuiltInType(BuiltInType::Float);
    // boolean_tag => Token::BuiltInType(BuiltInType::Boolean);
    // string_tag => Token::BuiltInType(BuiltInType::String);
    // symbol_tag => Token::BuiltInType(BuiltInType::Symbol);
    // void_tag => Token::BuiltInType(BuiltInType::Void);
    //
    // brace_open_tag => Token::Delimiter(Delimiter::BraceOpen);
    // brace_close_tag => Token::Delimiter(Delimiter::BraceClose);
    // bracket_open_tag => Token::Delimiter(Delimiter::BracketOpen);
    // bracket_close_tag => Token::Delimiter(Delimiter::BracketClose);
    // paren_open_tag => Token::Delimiter(Delimiter::ParenOpen);
    // paren_close_tag => Token::Delimiter(Delimiter::ParenClose);
    //
    // eof_tag => Token::EOF;
}

pub fn span(input: Input) -> PineResult<Span> {
    let (_, pos) = take(1usize)(input.clone())?;
    let a=input.clone();

    Ok((
        a,
        Span::from(pos)

    ))
}
pub fn spaned<'a, F, O1>(parser: F) -> impl FnMut(Input<'a>) -> PineResult<'a, Positioned<O1>>
    where
        F: Parser<Input<'a>, O1, Error<Input<'a>>>,
{
    map(
        tuple((span, parser, span)),
        |(start, result, end)| start.between(end).wrap(result),
    )
}

fn to_positioned(span: Input<'_>) -> Positioned<Input<'_>> {
    let input=span.clone();
    Positioned::new(span, Span::from(input))
}
pub fn ok_tag(input: Input<'_>) -> PineResult<Positioned<Input<'_>>> {

    // 使用 verify 进行验证
    verify(map(tag("ok"), |s: Input<'_>| to_positioned(s)), |pos: &Positioned<Input<'_>>| {
        println!("{:?}", pos.value.fragment());
        pos.value.fragment().to_string() == "ok" // 确保片段值为 "ok"
    })(input)


}