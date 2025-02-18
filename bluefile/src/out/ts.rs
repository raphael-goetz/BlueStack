use std::fs::File;
use std::io::Write;
use crate::{AttributeType, Node};

fn attribute_to_type(attribute_type: AttributeType) -> String {
    match attribute_type {
        AttributeType::Byte => String::from("number"),
        AttributeType::Int => String::from("number"),
        AttributeType::Float => String::from("number"),
        AttributeType::Boolean => String::from("boolean"),
        AttributeType::String => String::from("string"),
    }
}

pub fn write_ts(nodes: Vec<Node>) {
    let mut file = File::create_new("out.ts").expect("TaFad");

    for node in nodes {
        let header = format!("type {} = {{\n", node.name);
        let mut attributes: Vec<String> = Vec::new();

        for attribute in node.fields {
            let datatype = attribute_to_type(attribute.attribute_type);
            let line = format!(
                "\t{}: {}\n",
                attribute.name, datatype
            );
            attributes.push(line);
        }
        let _ = file.write(format!("{}", header).as_bytes());
        attributes.iter().for_each(|l| {
            file.write(format!("{}", l).as_bytes()).expect("err");
        });
        let _ = file.write("}".as_bytes());
    }
}