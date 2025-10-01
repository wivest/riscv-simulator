use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "rules.pest"]
pub struct Grammar;
