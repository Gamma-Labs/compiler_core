use std::vec::Vec;
use std::option::Option;

// list<Rule, Sep=","> := (Rule (Sep Rule)*)?

struct File { // (Declaration ";")*
    declarations: Vec<Declaration>,
}

enum Declaration {
    FunctionDeclaration(Identifier, TemplateParameters, Parameters, Type, CompoundStatement),
    // "fn" Identifier TemplateParameters Parameters "->" Type CompoundStatement
}

struct Identifier {
    name: String, // [a-z A-Z][a-z A-Z 0-9 _]*
}

struct TemplateParameters { // ("<" list<Parameter> ">")*
    lists: Vec<Vec<Parameter>>,
}

struct Parameters { // ("(" list<Parameter> ")")*
    lists: Vec<Vec<Parameter>>,
}

struct Type {
    expr: Expression,
}

struct CompoundStatement { // "{" (Statement ";")* "}"
    stmts: Vec<Statement>,
}

struct Parameter { // "mut"? Identifier ":" Type ("=" Expression)?
    mutable: Mutability,
    name: Identifier,
    type_: Type,
    default_value: Option<Expression>,
}

enum Expression {
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
    FnType(TemplateParameters, Parameters, Box<Type>),
    // "fn" TemplateParameters Parameters "->" type
    ArrayType(Box<Expression>, Box<Type>),
    // "[" Expression "]" Type

}

enum Statement {
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

enum Mutability {
    None,
    Mutable,
    Const,
}

struct Lambda { // "[" "]" TemplateParameters Parameters "->" Type CompoundStatement
    temp_params: TemplateParameters,
    params: Parameters,
    ret_type: Box<Type>,
    body: CompoundStatement,
}

enum Literal {
    False,
    True,
    // CharLiteral(CharLiteral),
    // StringLiteral(StringLiteral),
    FloatLiteral(FloatLiteral),
    IntegerLiteral(IntegerLiteral),
    ArrayLiteral(ArrayLiteral),
}

enum BracketType {
    Paren, // ()
    Square, // []
    Angle, // <>
    Brace, // ()
}

enum PrefixOperator {
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

enum InfixOperator {
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

struct FloatLiteral {
    value: String, // Digit+ "." Digit+ ("e" ("+" | "-")? Digit+)?
    suffix: FloatSuffix,
}

struct IntegerLiteral {
    prefix: IntegerPrefix,
    value: String,
    suffix: IntegerSuffix,
}

struct ArrayLiteral { // "[" Expression ("," Expression)* "]"
    elems: Vec<Expression>,
}

enum FloatSuffix {
    None,
    F32,
    F64,
}

enum IntegerPrefix {
    None,
    BinaryPrefix, // "0b"
    OctalPrefix, // "0o"
    HexPrefix, // "0x"
}

enum IntegerSuffix {
    None,
    U8, U16, U32, U64,
    I8, I16, I32, I64,
}
