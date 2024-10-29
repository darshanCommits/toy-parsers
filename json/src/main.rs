use std::fs;

use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct JSONParser;

enum JSONValue<'a> {
    Object(Vec<(&'a str, JSONValue<'a>)>),
    Array(Vec<JSONValue<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null,
}

fn json_serializer(val: &JSONValue) -> String {
    use JSONValue::*;

    match val {
        Object(x) => {
            let contents: Vec<_> = x
                .iter()
                .map(|(name, value)| format!("\"{}\":{}", name, json_serializer(value)))
                .collect();
            format!("{{{}}}", contents.join(","))
        }
        Array(x) => {
            let content: Vec<_> = x.iter().map(json_serializer).collect();
            format!("[{}]", content.join(","))
        }
        String(x) => format!("\"{}\"", x),
        Number(x) => x.to_string(),
        Boolean(x) => x.to_string(),
        Null => "null".to_string(),
    }
}

fn parse_json(file: &str) -> Result<JSONValue, Error<Rule>> {
    let json = JSONParser::parse(Rule::json, file)?.next().unwrap();

    use pest::iterators::Pair;
    fn parse_value(pair: Pair<'_, Rule>) -> JSONValue<'_> {
        match pair.as_rule() {
            Rule::object => JSONValue::Object(
                pair.into_inner()
                    .map(|x| {
                        let mut inner_rule = x.into_inner();
                        let key = inner_rule
                            .next()
                            .unwrap()
                            .into_inner()
                            .next()
                            .unwrap()
                            .as_str();
                        let value = parse_value(inner_rule.next().unwrap());
                        (key, value)
                    })
                    .collect(),
            ),
            Rule::array => JSONValue::Array(pair.into_inner().map(parse_value).collect()),
            Rule::string => JSONValue::String(pair.into_inner().next().unwrap().as_str()),
            Rule::number => JSONValue::Number(pair.as_str().parse().unwrap()),
            Rule::boolean => JSONValue::Boolean(pair.as_str().parse().unwrap()),
            Rule::null => JSONValue::Null,

            Rule::WHITESPACE
            | Rule::value
            | Rule::pair
            | Rule::inner
            | Rule::char
            | Rule::json
            | Rule::EOI => unreachable!(),
        }
    }

    Ok(parse_value(json))
}

fn main() {
    let raw_file = fs::read_to_string("data.json").expect("failed");
    let json = parse_json(&raw_file).expect("failed");

    println!("{}", json_serializer(&json));
}
