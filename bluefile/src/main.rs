pub mod grammar;
pub mod out;

use crate::grammar::{is_attribute, is_keyword, match_attribute, Node, NodeField, Token};
use std::fs::File;
use std::io::Read;

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
            'a'..='z' | 'A'..='Z' | '[' | ']' => {
                word.push(c);
                position += 1;
                it.next(); // Consume the character

                // Check for the next character to decide if we should end the word
                while let Some(&next_char) = it.peek() {
                    if next_char.is_alphanumeric()
                        || next_char == '_'
                        || next_char == '['
                        || next_char == ']'
                    {
                        word.push(next_char);
                        it.next(); // Consume the character
                    } else {
                        break; // Not part of the word anymore
                    }
                }

                // Build the string from the collected characters
                let real_word: String = word.iter().collect();

                let last_token_option = tokens.last();

                let last_token: &Token = match last_token_option {
                    Some(token) => token,
                    None => {
                        if !is_keyword(&real_word) {
                            panic!("Expected model at the beginning!")
                        }
                        &Token::Keyword(real_word.clone())
                    }
                };

                match last_token {
                    Token::Keyword(key) => {
                        //the name of the class
                        tokens.push(Token::Identifier(real_word));
                    }
                    Token::Identifier(_) => {
                        if is_attribute(&real_word) {
                            tokens.push(Token::Attribute(match_attribute(real_word)));
                        } else {
                            if real_word.ends_with("[]") {
                                tokens.push(Token::Attribute(grammar::AttributeType::CustomArray));
                            } else {
                                tokens.push(Token::Attribute(grammar::AttributeType::Custom));
                            }
                        }
                    }
                    _ => {
                        tokens.push(Token::Identifier(real_word));
                    }
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

fn parse(tokens: Vec<Token>) -> Result<Vec<Node>, ParseError> {
    let mut chunks: Vec<Vec<Token>> = Vec::new();
    let mut current_chunk: Vec<Token> = Vec::new();
    let mut nodes: Vec<Node> = Vec::new();

    let mut it = tokens.iter().peekable();
    let mut current = it.next();

    if current.is_none() {
        return Err(ParseError);
    }

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
        println!("c starting now");
        for c in chunk {
            println!("{:?}", c)
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
            if let (Token::Identifier(name), Token::Attribute(attribute_type)) =
                (&sub_chunk[i], &sub_chunk[i + 1])
            {
                let field = NodeField {
                    attribute_type: attribute_type.clone(),
                    is_nullable: false,
                    name: name.clone(),
                };

                fields.push(field);
                i += 2; // Move past this pattern
            } else {
                panic!("unexpected token!")
            }
        }

        let node = Node { name, fields };
        nodes.push(node);
    }

    Ok(nodes)
}

fn logic(nodes: Vec<Node>) {
    //check for duplicate fields

    for node in nodes {
        let names: Vec<String> = node.fields.iter().map(|f| f.name.clone()).collect();
        let mut duplicate_fields = Vec::new();
        let mut seen: Vec<String> = Vec::new();

        for name in names {
            if seen.contains(&name) {
                duplicate_fields.push(name);
            } else {
                seen.push(name);
            }
        }

        if !duplicate_fields.is_empty() {
            panic!("There are duplicate fields! {:?}", duplicate_fields)
        }
    }
}

fn main() {
    let mut file = File::open("example.bluefile").expect("Cannot open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Cannot read file");

    let tokens = lex(content);
    let nodes = parse(tokens);

    match nodes {
        Ok(real_node) => {
            for node in &real_node {
                println!("{:?}", node)
            }

            logic(real_node);
            //write_ts(real_node.clone());
            //write_go(real_node);
        }
        Err(_) => {}
    }
}
