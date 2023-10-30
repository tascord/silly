/*
    Actions:
        - <Value> <Operation> <Value> (e.g 1 + 1)
        - let <Identifier> = <Value> (e.g let x = 1)
        - <Identifier> = <Value> (e.g x = 1)
        - fn <Identifier> (<Identifier>, ...) { <Block> } (e.g fn add(x, y) { x + y })
        - io.<Identifier> (e.g io.system_name)
        - io.<Identifier>(<Value>) (e.g io.print("Hello, world"))
*/

/// Define basic nodes

// Ways of referring to a value
#[derive(PartialEq, Debug, Clone)]
pub struct NumberLiteral(pub f64);
#[derive(PartialEq, Debug, Clone)]
pub struct StringLiteral(pub String);
#[derive(PartialEq, Debug, Clone)]
pub struct IdentifierLiteral(pub String);

// A value
#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    NumberLiteral(NumberLiteral),
    StringLiteral(StringLiteral),
    IdentifierLiteral(IdentifierLiteral),
}

pub struct Call {
    pub name: IdentifierLiteral,
    pub arguments: Vec<Value>,
}

// Single operations that can be applied to values
#[derive(PartialEq, Debug, Clone)]
pub enum MonadicVerb {
    Negate,
    Increment,
    Decrement,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Monadic {
    pub verb: MonadicVerb,
    pub value: Box<AstNode>,
}

// Operators that can be applied to values
#[derive(PartialEq, Debug, Clone)]
pub enum DyadicVerb {
    // Assignment
    Assign,

    // Comparisons
    Equal,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

    // Maths
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Dyadic {
    pub verb: DyadicVerb,
    pub left: Box<AstNode>,
    pub right: Box<AstNode>,
}

// Calling a function
#[derive(PartialEq, Debug, Clone)]
pub struct FunctionCall {
    pub reference: Box<AstNode>,
    pub arguments: Vec<AstNode>,
}

// A statement
#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    // Literals
    NumberLiteral(f64),
    StringLiteral(String),
    IdentifierLiteral(String),

    // Operations
    Monadic(Monadic),
    Dyadic(Dyadic),

    // Function call
    FunctionCall(Box<AstNode>, Vec<AstNode>),

    Terms(Vec<AstNode>),
    Assignment(IdentifierLiteral, Box<AstNode>),
    Item(IdentifierLiteral, Vec<AstNode>),
    Call(Box<AstNode>, Vec<AstNode>),
}
