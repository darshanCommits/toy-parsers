use std::{collections::HashMap, fmt::Debug, fs};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct INIParser;

fn main() {
    let input = fs::read_to_string("example.ini").expect("failed to read");
    let file = INIParser::parse(Rule::file, &input)
        .unwrap()
        .next() // doing this as we get an array with single element (since we enclosed the grammar in a file rule)
        .unwrap();

    let mut properties: HashMap<&str, HashMap<&str, &str>> = HashMap::new();

    let mut current_section_name = "";

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::section => {
                let mut inner_rule = line.into_inner();
                current_section_name = inner_rule.next().unwrap().as_str();
            }
            Rule::property => {
                let mut inner_rule = line.into_inner();

                let key = inner_rule.next().unwrap().as_str();
                let value = inner_rule.next().unwrap().as_str();
                let section = properties.entry(current_section_name).or_default();
                section.insert(key, value);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    println!("{:#?}", properties);
}
