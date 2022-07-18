use std::vec::Vec;
use std::collections::HashMap;

use crate::ast::*;

pub struct Scope {
    pub parts: Vec<String>,
}

pub enum BorrowStatus {
    MutablyBorrowed,
    ImmutablyBorrowed(usize),
}

pub struct SymbolEntry {
    pub id: usize,
    pub name: String,
    pub scope: Scope,
    pub type_: Type,
    pub mutability: Mutability,
    // pub borrow_status: BorrowStatus
}

pub struct SymbolTable {
    pub table: Vec<SymbolEntry>,
}

pub enum ValueCategory {
    LValue,
    RValue,
}

pub enum ExpressionInfo {
    Lambda(String), // the name of the lambda's type
    Literal,
    Identifier(usize), // the id of the symbol in the symbol table
    Call(usize, Vec<usize>), //  the id's of the function to call and the arguments
    Borrow_(Mutability, usize), // the mutability of the borrow, and the id of the borrowed expression
}

pub struct ExpressionEntry {
    pub id: usize,
    pub type_: Type,
    pub mutability: Mutability,
    pub value_category: ValueCategory,
    pub info: ExpressionInfo,
}

pub struct ExpressionTable {
    pub table: HashMap<*const Expression, ExpressionEntry>,
}

pub struct Info {
    pub symbol_table: SymbolTable,
    pub expr_table: ExpressionTable,
}

impl Default for Info {

    fn default() -> Self {
        Info{
            symbol_table: SymbolTable{
                table: Vec::new(),
            },
            expr_table: ExpressionTable{
                table: HashMap::new(),
            },
        }
    }

}
