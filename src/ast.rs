use std::vec::Vec;
use std::option::Option;

// list<Rule, Sep=","> := (Rule (Sep Rule)*)?

pub struct File { // (Declaration ";")*
    pub declarations: Vec<Declaration>,
}

pub enum Declaration {
    FunctionDeclaration(Identifier, TemplateParameters, Parameters, Type, CompoundStatement),
    // "fn" Identifier TemplateParameters Parameters "->" Type CompoundStatement
}

pub struct Identifier {
    pub name: String, // [a-z A-Z][a-z A-Z 0-9 _]*
}

pub struct TemplateParameters { // ("<" list<Parameter> ">")*
    pub lists: Vec<Vec<Parameter>>,
}

pub struct Parameters { // ("(" list<Parameter> ")")*
    pub lists: Vec<Vec<Parameter>>,
}

pub enum Type {
    Named(Identifier),
    RefType(Mutability, Box<Type>),
    // "*" Mutability Type
    FnType(TemplateParameters, Parameters, Box<Type>),
    // "fn" TemplateParameters Parameters "->" type
    ArrayType(Box<Expression>, Box<Type>),
    // "[" Expression "]" Type
}

pub struct CompoundStatement { // "{" (Statement ";")* "}"
    pub stmts: Vec<Statement>,
}

pub struct Parameter { // "mut"? Identifier ":" Type ("=" Expression)?
    pub mutable: Mutability,
    pub name: Identifier,
    pub type_: Type,
    pub default_value: Option<Expression>,
}

pub enum Expression {
    // "(" Expression ")"
    Lambda(Lambda),
    Literal(Literal),
    Identifier(Identifier),
    Call(Box<Expression>, BracketType, Vec<Expression>),
    // Expression BracketType list<Expression> BracketType
    Prefix(PrefixOperator, Box<Expression>),
    // PrefixOperator Expression
    Infix(Box<Expression>, InfixOperator, Box<Expression>),
    // Expression InfixOperator Expression

    /*
    FnType(TemplateParameters, Parameters, Box<Type>),
    // "fn" TemplateParameters Parameters "->" type
    ArrayType(Box<Expression>, Box<Type>),
    // "[" Expression "]" Type
    */
}

pub enum Statement {
    CompoundStatement(CompoundStatement),
    LetStatement(Mutability, Identifier, Option<Type>, Expression),
    // "let" "mut"? Identifier (":" Type)? "=" Expression
    IfStatement(Expression, Option<CompoundStatement>, Box<Statement>),
    // "if" "(" Expression ")" (CompoundStatement "else")? Statement
    WhileStatement(Expression, Box<Statement>),
    // "while" "(" Expression ")" Statement
    BreakStatement,
    ContinueStatement,
    ReturnStatement(Expression),
    ExpressionStatement(Expression),
}

pub enum Mutability {
    None,
    Mutable,
    Const,
}

pub struct Lambda { // "[" "]" TemplateParameters Parameters "->" Type CompoundStatement
    pub temp_params: TemplateParameters,
    pub params: Parameters,
    pub ret_type: Box<Type>,
    pub body: CompoundStatement,
}

pub enum Literal {
    // CharLiteral(CharLiteral),
    // StringLiteral(StringLiteral),
    FloatLiteral(FloatLiteral),
    IntegerLiteral(IntegerLiteral),
    ArrayLiteral(ArrayLiteral),
}

pub enum BracketType {
    Paren, // ()
    Square, // []
    Angle, // <>
    Brace, // ()
}

pub enum PrefixOperator {
    Increment,
    Decrement,
    Plus,
    Minus,
    Exclamation,
    Tilde,
    Deref_, // "*"
    Borrow_, // "&"
    BorrowMut_, // "&" "mut"
}

pub enum InfixOperator {
    // each line is higher precedence than the next
    Times, Divide, Modulo,
    Plus, Minus,
    LeftShift, RightShift,
    Less, LessEqual, GreaterEqual, Greater,
    DoubleEqual, NotEqual,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LogicalAnd,
    LogicalOr,
    // except every assignment operator has the same precedence
    Assign,
    TimesEquals, DivideEquals, ModuloEquals,
    PlusEquals, MinusEquals,
    LeftShiftEquals, RightShiftEquals,
    BitwiseAndEquals, BitwiseXorEquals, BitwiseOrEquals,
}

pub struct FloatLiteral {
    pub value: String, // Digit+ "." Digit+ ("e" ("+" | "-")? Digit+)?
    pub suffix: FloatSuffix,
}

pub struct IntegerLiteral {
    pub prefix: IntegerPrefix,
    pub value: String,
    pub suffix: IntegerSuffix,
}

pub struct ArrayLiteral { // "[" Expression ("," Expression)* "]"
    pub elems: Vec<Expression>,
}

pub enum FloatSuffix {
    None,
    F32,
    F64,
}

pub enum IntegerPrefix {
    None,
    BinaryPrefix, // "0b"
    OctalPrefix, // "0o"
    HexPrefix, // "0x"
}

pub enum IntegerSuffix {
    None,
    U8, U16, U32, U64,
    I8, I16, I32, I64,
}
