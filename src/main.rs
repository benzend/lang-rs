use regex::Regex;

pub const EXAMPLE: &'static str = "
if true {
    print 'true'
} else {
    print 'false'
}
";

const REGEX: &'static str = r"((?:-|\b)\d+\b|[=;{}]|\[\]|\[deprecated\]|\b[A-Za-z_][A-Za-z0-9_]*\b|\/\/.*|\s+)";
const IDENTIFIER: &'static str = r"/^[A-Za-z_][A-Za-z0-9_]*$";
const WHITESPACE: &'static str = r"/^\/\/.*|\s+$";
const EQUALS: &'static str = r"/^=$/";
const END_OF_FILE: &'static str = r"/^$/";
const SEMICOLON: &'static str = r"/^;$/";
const INTEGER: &'static str = r"/^-?\d+$/";
const LEFT_BRACE: &'static str = r"/^\{$/";
const RIGHT_BRACE: &'static str = r"/^\}$/";
const ARRAY_TOKEN: &'static str = r"/^\[\]$/";
const ENUM_KEYWORD: &'static str = r"/^enum$/";
const STRUCT_KEYWORD: &'static str = r"/^struct$/";

fn main() {
    let tokens = tokenize(EXAMPLE);

    println!("{:?}", tokens);
}

#[derive(Debug)]
pub enum Token {
    Identifier,
    Equals,
    EndOfFile,
    Semicolon,
    Integer,
    LeftBrace,
    RightBrace,
    ArrayToken,
    EnumKeyword,
    StructKeyword
}

struct TokenData {
    token: Token,
    line: u128,
    column: u128
}

pub fn tokenize(text: &str) -> Result<Vec<Token>, std::io::Error> {
    let reg_parse_text = Regex::new(REGEX).unwrap();
    let reg_identifier = Regex::new(IDENTIFIER).unwrap();
    let reg_whitespace = Regex::new(WHITESPACE).unwrap();
    let req_equals = Regex::new(EQUALS).unwrap();
    let reg_end_of_file = Regex::new(END_OF_FILE).unwrap();
    let reg_semicolon = Regex::new(SEMICOLON).unwrap();
    let reg_integer = Regex::new(INTEGER).unwrap();
    let reg_left_brace = Regex::new(LEFT_BRACE).unwrap();
    let reg_right_brace = Regex::new(RIGHT_BRACE).unwrap();
    let reg_array_token = Regex::new(ARRAY_TOKEN).unwrap();
    let reg_enum_keyword = Regex::new(ENUM_KEYWORD).unwrap();
    let reg_struct_keyword = Regex::new(STRUCT_KEYWORD).unwrap();

    let mut tokens: Vec<TokenData> = Vec::new();

    let mut column = 0;
    let mut line = 0;

    let mut results: Vec<&str> = Vec::new();
    for (_, [part])  in reg_parse_text.captures_iter(text).map(|c| c.extract()) {
        println!("part: {}", part);
        results.push(part);
    }

    for (i, part) in results.iter().enumerate() {

    }

    Ok(Vec::new())
}
