use pest::Parser;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "dcm.pest"]
pub struct DCMParser;

pub fn parse() {
    let unparsed_file = fs::read_to_string("test.dcm").expect("cannot read file");

    let file = DCMParser::parse(Rule::dcm, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails
}