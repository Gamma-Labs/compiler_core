use std::vec::Vec;
use std::collections::HashMap;

use crate::ast::*;

#[derive(Clone, Default)]
pub struct Scope {
    pub parts: Vec<String>,
}

#[derive(Copy, Clone)]
pub enum BorrowStatus {
    MutablyBorrowed,
    ImmutablyBorrowed(usize),
}

#[derive(Clone)]
pub struct SymbolEntry {
    pub id: usize,
    pub name: String,
    pub scope: Scope,
    pub type_: Type,
    pub mutability: Mutability,
    // pub borrow_status: BorrowStatus
}

#[derive(Clone, Default)]
pub struct SymbolTable {
    pub table: Vec<SymbolEntry>,
}

impl SymbolTable {
    pub fn lookup(&self, ident: &Identifier, scope: &Scope) -> &SymbolEntry {
        for entry in self.table.iter().rev() {
            if entry.name == ident.name {
                if scope.parts.starts_with(&entry.scope.parts) {
                    return &entry;
                }
            }
        }
        panic!("Identifier Not Found")
    }
}

#[derive(Copy, Clone)]
pub enum ValueCategory {
    LValue,
    RValue,
}

#[derive(Clone)]
pub enum ExpressionInfo {
    Lambda(usize), // the id of the lambda's type
    Literal,
    Identifier(usize), // the id of the symbol in the symbol table
    Call(usize, Vec<usize>), //  the id's of the function to call and the arguments
    Borrow_(Mutability, usize), // the mutability of the borrow, and the id of the borrowed expression
}

#[derive(Clone)]
pub struct ExpressionEntry {
    pub id: usize,
    pub type_: Type,
    pub mutability: Mutability,
    pub value_category: ValueCategory,
    pub info: ExpressionInfo,
}

#[derive(Clone, Default)]
pub struct ExpressionTable {
    pub table: HashMap<*const Expression, ExpressionEntry>,
}

impl ExpressionTable {
    pub fn lookup(&self, expr: &Expression) -> &ExpressionEntry {
        return &self.table[&(expr as *const Expression)];
    }
}

#[derive(Clone, Default)]
pub struct Info {
    pub symbol_table: SymbolTable,
    pub expr_table: ExpressionTable,
}
