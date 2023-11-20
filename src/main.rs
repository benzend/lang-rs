use regex::{Error, Regex};

pub const EXAMPLE: &'static str = "
if true {
    print 'true'
} else {
    print 'false'
}
";

const REGEX: &'static str =
    r"((?:-|\b)\d+\b|[=;{}]|\[\]|\[deprecated\]|\b[A-Za-z_][A-Za-z0-9_]*\b|\/\/.*|\s+)";
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
    let parser = Parser::new().unwrap();
    let tokens = parser.tokenize(EXAMPLE);

    println!("{:?}", tokens);
}

pub struct Parser {
    reg_split: Regex,
    reg_identifier: Regex,
    reg_whitespace: Regex,
    reg_equals: Regex,
    reg_end_of_file: Regex,
    reg_semicolon: Regex,
    reg_integer: Regex,
    reg_left_brace: Regex,
    reg_right_brace: Regex,
    reg_array_token: Regex,
    reg_enum_keyword: Regex,
    reg_struct_keyword: Regex,
}

impl Parser {
    pub fn new() -> Result<Parser, Error> {
        Ok(Parser {
            reg_split: Regex::new(REGEX).unwrap(),
            reg_identifier: Regex::new(REGEX).unwrap(),
            reg_whitespace: Regex::new(REGEX).unwrap(),
            reg_equals: Regex::new(REGEX).unwrap(),
            reg_end_of_file: Regex::new(REGEX).unwrap(),
            reg_semicolon: Regex::new(REGEX).unwrap(),
            reg_integer: Regex::new(REGEX).unwrap(),
            reg_left_brace: Regex::new(REGEX).unwrap(),
            reg_right_brace: Regex::new(REGEX).unwrap(),
            reg_array_token: Regex::new(REGEX).unwrap(),
            reg_enum_keyword: Regex::new(REGEX).unwrap(),
            reg_struct_keyword: Regex::new(REGEX).unwrap(),
        })
    }

    pub fn tokenize(&self, buffer: &str) -> Result<Vec<Token>, std::io::Error> {
        let mut tokens: Vec<Token> = Vec::new();

        let mut column = 0;
        let mut line = 0;

        let mut results: Vec<Token> = Vec::new();
        for (_, [part]) in self.reg_split.captures_iter(buffer).map(|c| c.extract()) {
            println!("part: {}", part);
            if self.reg_identifier.is_match(part) {
                if self.reg_struct_keyword.is_match(part) {
                    tokens.push(Token::StructKeyword(Column(1), Line(1)))
                }
            }
        }

        for (i, part) in results.iter().enumerate() {}

        Ok(tokens)
    }
}

#[derive(Debug)]
pub enum Token {
    Identifier(Column, Line),
    Equals(Column, Line),
    EndOfFile(Column, Line),
    Semicolon(Column, Line),
    Integer(Column, Line),
    LeftBrace(Column, Line),
    RightBrace(Column, Line),
    ArrayToken(Column, Line),
    EnumKeyword(Column, Line),
    StructKeyword(Column, Line),
}

#[derive(Debug)]
pub struct Column(usize);
#[derive(Debug)]
pub struct Line(usize);
