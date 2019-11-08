extern crate pest;
use pest::error::Error;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammars/cultures.pest"]
pub struct CultureParser;

use crate::json::JSONValue;

pub fn parse(culture_text: &str) -> Result<JSONValue, Error<Rule>> {
    use pest::iterators::Pair;

    let parsing_iter = CultureParser::parse(Rule::file, culture_text)
        .expect("could not parse culture grammar")
        .next()
        .unwrap();

    fn parse_inequality(pair: pest::iterators::Pair<'_, Rule>) -> JSONValue {
        let mut inner_pairs = pair.into_inner();

        let inner_identifier = inner_pairs
            .next()
            .expect("parse_inequality: inner_identifier was None");
        match inner_identifier.as_rule() {
            Rule::identifier => (),
            _ => unreachable!("inequality did not match inner_identifier")
        }
        let name = parse_value(inner_identifier);

        let inner_op = inner_pairs
            .next()
            .expect("parse_inequality: inner_op was None");
        match inner_op.as_rule() {
            Rule::inequality_operator => (),
            _ => unreachable!("inequality did not match inner_op")
        }
        let op = parse_value(inner_op);

        let inner_value = inner_pairs
            .next()
            .expect("parse_inequality: inner_value was None");
        let value = parse_value(inner_value);
        
        JSONValue::Object(vec![("__inequality_key", name), ("__inequality_op", op), ("__inequality_value", value)])
    }

    fn parse_pair(pair: pest::iterators::Pair<'_, Rule>) -> (&str, JSONValue<'_>) {
        let mut inner_pairs = pair.into_inner();

        let inner_identifier = inner_pairs
            .next()
            .expect("parse_pair: inner_identifier was None");
        match inner_identifier.as_rule() {
            Rule::identifier => (),
            _ => unreachable!("pair did not match inner_identifier")
        }
        let name = inner_identifier.as_str();

        let inner_value = inner_pairs
            .next()
            .expect("parse_pair: inner_value was None");
        let value = parse_value(inner_value);
        
        (name, value)
    }

    fn parse_value(pair: Pair<Rule>) -> JSONValue {
        match pair.as_rule() {
            Rule::object
            | Rule::body => JSONValue::Object(
                pair.into_inner()
                    .map(|pair| {
                        match pair.as_rule() {
                            Rule::object
                            | Rule::pair => parse_pair(pair),
                            | Rule::inequality => ("__inequality", parse_inequality(pair)),
                            _ => {
                                println!("Rule:    {:?}", pair.as_rule());
                                println!("Span:    {:?}", pair.as_span());
                                println!("Text:    {}", pair.as_str());
                                unreachable!()
                            }
                        }
                    })
                    .collect(),
            ),
            Rule::array => JSONValue::Array(pair.into_inner().map(parse_value).collect()),
            Rule::string
            | Rule::date => JSONValue::String(pair.into_inner().next().unwrap().as_str()),
            Rule::identifier
            | Rule::tag
            | Rule::inequality_operator => JSONValue::String(pair.as_str()),
            Rule::number => JSONValue::Number(pair.as_str().parse().unwrap()),
            Rule::boolean => JSONValue::Boolean(pair.as_str() == "yes"),
            Rule::file
            | Rule::EOI
            | Rule::pair
            | Rule::inequality
            | Rule::value
            | Rule::inner
            | Rule::char
            | Rule::int
            | Rule::float
            | Rule::date_inner
            | Rule::comment_inner
            | Rule::COMMENT
            | Rule::WHITESPACE => {
                println!("Rule:    {:?}", pair.as_rule());
                println!("Span:    {:?}", pair.as_span());
                println!("Text:    {}", pair.as_str());
                unreachable!()
            }
        }
    }

    Ok(parse_value(parsing_iter))
}
