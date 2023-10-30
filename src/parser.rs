use pest::iterators::Pair;
use pest::{error::Error, Parser};

#[derive(pest_derive::Parser)]
#[grammar = "./grammar.pest"]
struct FloraParser;

use crate::types::{
    AstNode::{self, *},
    Dyadic, DyadicVerb, IdentifierLiteral, Monadic, MonadicVerb,
};

pub fn parse_monadic_verb(pair: Pair<Rule>, expression: AstNode) -> AstNode {
    AstNode::Monadic(Monadic {
        verb: match pair.as_str() {
            "-" => MonadicVerb::Negate,
            "++" => MonadicVerb::Increment,
            "--" => MonadicVerb::Decrement,
            _ => panic!("Unknown monadic verb: {}", pair.as_str()),
        },
        value: Box::new(expression),
    })
}

pub fn parse_dyadic_verb(pair: Pair<Rule>, left: AstNode, right: AstNode) -> AstNode {
    AstNode::Dyadic(Dyadic {
        left: Box::new(left),
        right: Box::new(right),
        verb: match pair.as_str() {
            "+" => DyadicVerb::Add,
            "-" => DyadicVerb::Subtract,
            "*" => DyadicVerb::Multiply,
            "/" => DyadicVerb::Divide,
            "==" => DyadicVerb::Equal,
            "<" => DyadicVerb::LessThan,
            "<=" => DyadicVerb::LessThanOrEqual,
            ">" => DyadicVerb::GreaterThan,
            ">=" => DyadicVerb::GreaterThanOrEqual,
            _ => panic!("Unknown dyadic verb: {}", pair.as_str()),
        },
    })
}

pub fn build_ast_from_term(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::numberLiteral => NumberLiteral(pair.as_str().parse::<f64>().unwrap()),
        Rule::stringLiteral => StringLiteral(pair.as_str().to_string()),
        Rule::expression => build_ast_from_expression(pair),
        Rule::identifier => AstNode::IdentifierLiteral(pair.as_str().to_string()),
        Rule::item => {
            let mut pair = pair.into_inner();
            let identifier = pair.next().unwrap();
            let mut arguments = Vec::new();
            while let Some(argument) = pair.next() {
                arguments.push(build_ast_from_expression(argument));
            }
            AstNode::Item(
                IdentifierLiteral(identifier.as_str().to_string()),
                arguments,
            )
        }
        Rule::call => {
            let mut pairs = pair.into_inner();
            let identifier = pairs.next().unwrap();

            let mut arguments: Vec<AstNode> = Vec::new();
            while let Some(argument) = pairs.next() {
                arguments.push(build_ast_from_expression(argument));
            }

            AstNode::FunctionCall(Box::new(build_ast_from_expression(identifier)), arguments)
        }
        _ => panic!("Unknown term {:?}: {:?}", pair.as_rule(), pair.as_span()),
    }
}

pub fn build_ast_from_expression(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::expression => build_ast_from_expression(pair.into_inner().next().unwrap()),
        Rule::call | Rule::item | Rule::identifier => build_ast_from_term(pair),
        Rule::monadic => {
            let mut pair = pair.into_inner();
            let verb = pair.next().unwrap();
            let expression = build_ast_from_expression(pair.next().unwrap());
            parse_monadic_verb(verb, expression)
        }
        Rule::dyadic => {
            let mut pair = pair.into_inner();
            let left_pair = pair.next().unwrap();
            let left = build_ast_from_expression(left_pair);
            let verb = pair.next().unwrap();
            let right_pair = pair.next().unwrap();
            let right = build_ast_from_expression(right_pair);
            parse_dyadic_verb(verb, left, right)
        }
        Rule::terms => {
            let terms: Vec<AstNode> = pair.into_inner().map(build_ast_from_term).collect();
            match terms.len() {
                1 => terms[0].clone(),
                _ => Terms(terms),
            }
        }
        Rule::assignment => {
            let mut pair = pair.into_inner();
            let identifier = pair.next().unwrap();
            let expression = pair.next().unwrap();
            AstNode::Assignment(
                IdentifierLiteral(identifier.as_str().to_string()),
                Box::new(build_ast_from_expression(expression)),
            )
        }
        _ => panic!("Unknown expression {:?}: {:?}", pair.as_rule(), pair),
    }
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];
    let pairs = FloraParser::parse(Rule::program, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::expression => {
                ast.push(build_ast_from_expression(pair));
            }
            _ => {}
        }
    }
    Ok(ast)
}
