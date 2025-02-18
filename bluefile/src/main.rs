mod out;

use crate::out::go::{write_go};
use std::cmp::PartialEq;
use crate::out::ts::write_ts;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    name: String,
    fields: Vec<NodeField>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NodeField {
    attribute_type: AttributeType,
    is_nullable: bool,
    name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AttributeType {
    Byte,
    Int,
    Float,
    Boolean,
    String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Attribute(AttributeType),
    LBracket,
    RBracket,
    Comma,
}

pub fn is_keyword(input: &String) -> bool {
    input == "model"
}

pub fn is_attribute(input: &String) -> bool {
    input == "byte" || input == "int" || input == "float" || input == "boolean" || input == "string"
}

pub fn match_attribute(input: String) -> AttributeType {
    match input.as_str() {
        "byte" => AttributeType::Byte,
        "boolean" => AttributeType::Boolean,
        "int" => AttributeType::Int,
        "float" => AttributeType::Float,
        "string" => AttributeType::String,
        _ => panic!("Unexpected attribute! {}", input),
    }
}

struct ParseError;
struct LexError;

fn lex(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut word: Vec<char> = Vec::new();
    let mut it = input.chars().peekable();

    let mut line = 1;
    let mut position = 1;

    while let Some(&c) = it.peek() {
        match c {
            'a'..='z' | 'A'..='Z' => {
                word.push(c);
                position += 1;
                it.next(); // Consume the character

                // Check for the next character to decide if we should end the word
                while let Some(&next_char) = it.peek() {
                    if next_char.is_alphanumeric() || next_char == '_' {
                        word.push(next_char);
                        it.next(); // Consume the character
                    } else {
                        break; // Not part of the word anymore
                    }
                }

                // Build the string from the collected characters
                let real_word: String = word.iter().collect();

                // Check if it's a keyword, attribute, or identifier
                if is_keyword(&real_word) {
                    tokens.push(Token::Keyword(real_word));
                } else if is_attribute(&real_word) {
                    tokens.push(Token::Attribute(match_attribute(real_word)));
                } else {
                    tokens.push(Token::Identifier(real_word));
                }
                word.clear();
            }
            '{' => {
                position += 1;
                it.next();
                tokens.push(Token::LBracket);
            }
            '}' => {
                position += 1;
                it.next();
                tokens.push(Token::RBracket);
            }
            ',' => {
                position += 1;
                it.next();
                tokens.push(Token::Comma);
            }
            ' ' => {
                position += 1;
                it.next();
            }
            '\n' => {
                it.next();
                position = 0;
                line += 1;
            }
            _ => {
                panic!(
                    "Unexpected character '{}' at line: {} position: {}, ",
                    c, line, position
                )
            }
        }
    }

    tokens
}

impl PartialEq<Token> for &Token {
    fn eq(&self, other: &Token) -> bool {
        match (self, other) {
            (Token::Keyword(..), Token::Keyword(..)) => true,
            (Token::Identifier(..), Token::Identifier(..)) => true,
            (Token::Attribute(..), Token::Attribute(..)) => true,
            (Token::LBracket, Token::LBracket) => true,
            (Token::RBracket, Token::RBracket) => true,
            (Token::Comma, Token::Comma) => true,
            _ => false,
        }
    }
}

fn parse(tokens: Vec<Token>) -> Result<Vec<Node>, ParseError> {
    let mut chunks: Vec<Vec<Token>> = Vec::new();
    let mut current_chunk: Vec<Token> = Vec::new();
    let mut nodes: Vec<Node> = Vec::new();

    for token in tokens {
        if token == Token::RBracket {
            current_chunk.push(Token::RBracket);
            chunks.push(current_chunk);
            current_chunk = Vec::new();
        } else {
            current_chunk.push(token);
        }
    }

    for chunk in &chunks {
        let last_index = &chunk.len() - 1;
        let expected_keyword = chunk[0].clone();
        let expected_identifier = chunk[1].clone();
        let expected_lbracket = chunk[2].clone();
        let expected_rbracket = chunk[last_index].clone();

        let mut name = String::new();

        match expected_keyword {
            Token::Keyword(_) => {}
            _ => {
                panic!("Expected Keyword at the beginning")
            }
        };

        match expected_identifier {
            Token::Identifier(identifier) => {
                name = identifier;
            }
            _ => {
                panic!("Expected Identifier after the Keyword!")
            }
        };

        println!("Last thing {:?}", expected_rbracket.clone());

        if expected_lbracket != Token::LBracket {
            panic!("Expected a opening bracket");
        }

        if expected_rbracket != Token::RBracket {
            println!("Didnt found clsing bracket");
            panic!("Expected a closing bracket");
        }

        let mut sub_chunk = &chunk[3..last_index];
        let mut fields: Vec<NodeField> = Vec::new();
        let mut i = 0;
        while i < sub_chunk.len() {
            if let (Token::Identifier(name), Token::Attribute(attribute_type), Token::Comma) =
                (&sub_chunk[i], &sub_chunk[i + 1], &sub_chunk[i + 2])
            {
                let field = NodeField {
                    attribute_type: attribute_type.clone(),
                    is_nullable: false,
                    name: name.clone(),
                };

                fields.push(field);
                i += 3; // Move past this pattern
            } else {
                panic!("unexpected token!")
            }
        }

        let node = Node { name, fields };
        nodes.push(node);
    }

    Ok(nodes)
}

fn main() {
    let code = r#"
    model Movie {
        id int,
        name string,
        star int,
    }
    "#;

    let tokens = lex(code.to_string());
    let nodes = parse(tokens);

    match nodes {
        Ok(real_node) => {
            for node in &real_node {
                println!("{:?}", node)
            }
            write_ts(real_node.clone());
            write_go(real_node);
        }
        Err(_) => {}
    }
}
