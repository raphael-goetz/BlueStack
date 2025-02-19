use crate::grammar::{AttributeType, Node};
use std::fs::File;
use std::io::Write;

fn attribute_to_type(attribute_type: AttributeType) -> String {
    match attribute_type {
        AttributeType::Byte | AttributeType::ByteArray => String::from("number"),
        AttributeType::Int | AttributeType::IntArray => String::from("number"),
        AttributeType::Float | AttributeType::FloatArray => String::from("number"),
        AttributeType::Boolean | AttributeType::BooleanArray => String::from("boolean"),
        AttributeType::String | AttributeType::StringArray => String::from("string"),
        AttributeType::Custom | AttributeType::CustomArray => String::from("value"),
    }
}

pub fn write_ts(nodes: Vec<Node>) {
    let mut file = File::create_new("out.ts").expect("TaFad");

    for node in nodes {
        let header = format!("type {} = {{\n", node.name);
        let mut attributes: Vec<String> = Vec::new();

        for attribute in node.fields {
            let datatype = attribute_to_type(attribute.attribute_type);
            let line = format!("\t{}: {}\n", attribute.name, datatype);
            attributes.push(line);
        }
        let _ = file.write(format!("{}", header).as_bytes());
        attributes.iter().for_each(|l| {
            file.write(format!("{}", l).as_bytes()).expect("err");
        });
        let _ = file.write("}".as_bytes());
    }
}
