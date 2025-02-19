use std::env::consts::ARCH;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub(crate) name: String,
    pub(crate) fields: Vec<NodeField>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NodeField {
    pub(crate) attribute_type: AttributeType,
    pub(crate) is_nullable: bool,
    pub(crate) name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AttributeType {
    Byte,
    ByteArray,
    Int,
    IntArray,
    Float,
    FloatArray,
    Boolean,
    BooleanArray,
    String,
    StringArray,
    Custom,
    CustomArray,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Attribute(AttributeType),
    LBracket,
    RBracket,
}

pub fn is_keyword(input: &String) -> bool {
    input == "model"
}

pub fn is_attribute(input: &String) -> bool {
    input == "byte"
        || input == "int"
        || input == "float"
        || input == "boolean"
        || input == "string"
        || input == "byte[]"
        || input == "int[]"
        || input == "float[]"
        || input == "boolean[]"
        || input == "string[]"
}

pub fn match_attribute(input: String) -> AttributeType {
    let option: Option<AttributeType> = match input.as_str() {
        "byte" => Some(AttributeType::Byte),
        "byte[]" => Some(AttributeType::ByteArray),
        "boolean" => Some(AttributeType::Boolean),
        "boolean[]" => Some(AttributeType::BooleanArray),
        "int" => Some(AttributeType::Int),
        "int[]" => Some(AttributeType::IntArray),
        "float" => Some(AttributeType::Float),
        "float[]" => Some(AttributeType::FloatArray),
        "string" => Some(AttributeType::String),
        "string[]" => Some(AttributeType::StringArray),
        _ => None,
    };

    if option.is_none() {
        if input.ends_with("[]") {
            return AttributeType::CustomArray;
        } else {
            return AttributeType::Custom;
        }
    };

    match option {
        Some(res) => res,
        None => panic!("AHAHAHAHAH"),
    }
}

impl PartialEq<Token> for &Token {
    fn eq(&self, other: &Token) -> bool {
        match (self, other) {
            (Token::Keyword(..), Token::Keyword(..)) => true,
            (Token::Identifier(..), Token::Identifier(..)) => true,
            (Token::Attribute(..), Token::Attribute(..)) => true,
            (Token::LBracket, Token::LBracket) => true,
            (Token::RBracket, Token::RBracket) => true,
            _ => false,
        }
    }
}
