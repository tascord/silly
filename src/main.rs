pub mod parser;
pub mod types;

// static IGNORED_RULES: [Rule; 3] = [Rule::EOI, Rule::COMMENT, Rule::WHITESPACE];

pub fn main() {
    let file = std::fs::read_to_string("./example.fl").unwrap();
    let ast = parser::parse(&file).unwrap_or_else(|error| {
        panic!("{}", error)
    });

    println!("{:?}", &ast);
}