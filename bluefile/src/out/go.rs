use crate::{AttributeType, Node};
use std::fs::File;
use std::io::Write;

fn attribute_to_type(attribute_type: AttributeType) -> String {
    match attribute_type {
        AttributeType::Byte => String::from("byte"),
        AttributeType::Int => String::from("int"),
        AttributeType::Float => String::from("float32"),
        AttributeType::Boolean => String::from("bool"),
        AttributeType::String => String::from("string"),
    }
}

pub fn write_go(nodes: Vec<Node>) {
    let mut file = File::create_new("out.go").expect("TaFad");
    let mut perms = file.metadata().expect("Efafdeaf").permissions();
    perms.set_readonly(true);
    let _ = file.write("package models \n\n".as_bytes());

    for node in nodes {
        let header = format!("type {} struct {{\n", node.name);
        let mut attributes: Vec<String> = Vec::new();

        for attribute in node.fields {
            let datatype = attribute_to_type(attribute.attribute_type);
            let line = format!(
                "\t{} {} `json:\"{}\"`\n",
                attribute.name, datatype, attribute.name
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
