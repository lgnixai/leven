use nom::{
    IResult,
    character::complete::{char, digit1, space0, multispace1},
    sequence::{delimited, preceded, terminated},
    bytes::complete::{take_while1},
    combinator::{map, opt},
    multi::{separated_list0},
};
use nom::bytes::complete::tag;
use nom::sequence::tuple;

// Helper parsers
fn identifier(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphabetic() || c == '_')(input)
}

fn integer(input: &str) -> IResult<&str, i32> {
    map(digit1, |digits: &str| digits.parse::<i32>().unwrap())(input)
}

fn parameter_definition(input: &str) -> IResult<&str, (&str, Option<i32>)> {
    let (input, id) = identifier(input)?;
    let (input, default_value) = opt(preceded(space0, preceded(char('=') , integer)))(input)?;
    Ok((input, (id, default_value)))
}

// Parse function signature
fn function_signature(input: &str) -> IResult<&str, (&str, Vec<(&str, Option<i32>)>)> {
    let (input, name) = identifier(input)?;
    let (input, params) = delimited(
        char('('),
        separated_list0(char(','), preceded(space0, parameter_definition)),
        char(')')
    )(input)?;
    Ok((input, (name, params)))
}

// Parse function body
fn function_body(input: &str) -> IResult<&str, Vec<&str>> {

    println!("body{:?}",input);
    let (input, _) = preceded(tuple((space0, tag("=>"), space0)), char('\n'))(input)?;

    println!("body{:?}",input);

    let mut lines = Vec::new();
    let mut remaining = input;
    while !remaining.trim_start().is_empty() {
        let (input, line) = delimited(space0, take_while1(|c| c != '\n'), char('\n'))(remaining)?;
        lines.push(line.trim());
        remaining = input;
    }
    Ok((remaining, lines))
}

// Parse the entire function
fn parse_function(input: &str) -> IResult<&str, String> {
    let (input, (name, params)) = function_signature(input)?;
    let (input, body) = function_body(input)?;

    let param_strs: Vec<String> = params
        .into_iter()
        .map(|(id, default)| {
            if let Some(value) = default {
                format!("{}: number = {}", id, value)
            } else {
                format!("{}", id)
            }
        })
        .collect();

    // Remove '=>' and use the last line of the body as the return value
    let body_str: Vec<&str> = body.iter().map(|line| *line).collect();
    let (body_lines, return_value) = body_str.split_at(body_str.len() - 1);

    let body_str = body_lines.join("\n    ");
    let return_value_str = return_value.first().unwrap_or(&"").to_string();

    // Construct TypeScript function
    let ts_function = format!(
        "function {}({}) {{\n    {}\n    return {};\n}}",
        name,
        param_strs.join(", "),
        body_str,
        return_value_str
    );

    Ok((input, ts_function))
}

#[test]
fn main() {
    let pine_script_code = r#"geom_average(x, y=6) =>
    a = x * x
    b = y * y
    math.sqrt(a + b)
"#;

    match parse_function(pine_script_code) {
        Ok((_, ts_code)) => println!("{}", ts_code),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
