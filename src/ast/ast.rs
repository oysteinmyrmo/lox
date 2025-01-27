use crate::scanner::token::{Literal as TokenLiteral, Token};

pub enum Expr {
    Empty,
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Box<Literal>),
    Unary(Box<Unary>),
}

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct Grouping {
    pub expression: Box<Expr>,
}

pub struct Literal {
    pub literal: TokenLiteral,
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub trait AstVisitor<T> {
    fn visit_binary(&mut self, visitor: &Binary) -> T;
    fn visit_grouping(&mut self, visitor: &Grouping) -> T;
    fn visit_literal(&mut self, visitor: &Literal) -> T;
    fn visit_unary(&mut self, visitor: &Unary) -> T;
}

pub trait Accept<T> {
    fn accept<V: AstVisitor<T>>(&self, visitor: &mut V) -> T;
}

impl<T> Accept<T> for Expr {
    fn accept<V: AstVisitor<T>>(&self, visitor: &mut V) -> T {
        match self {
            Expr::Empty => {
                panic!("Cannot visit Empty expression!")
            }
            Expr::Binary(binary) => visitor.visit_binary(binary),
            Expr::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expr::Literal(literal) => visitor.visit_literal(literal),
            Expr::Unary(unary) => visitor.visit_unary(unary),
        }
    }
}

impl<T> Accept<T> for Binary {
    fn accept<V: AstVisitor<T>>(&self, visitor: &mut V) -> T {
        visitor.visit_binary(self)
    }
}

impl<T> Accept<T> for Grouping {
    fn accept<V: AstVisitor<T>>(&self, visitor: &mut V) -> T {
        visitor.visit_grouping(self)
    }
}

impl<T> Accept<T> for Literal {
    fn accept<V: AstVisitor<T>>(&self, visitor: &mut V) -> T {
        visitor.visit_literal(self)
    }
}

impl<T> Accept<T> for Unary {
    fn accept<V: AstVisitor<T>>(&self, visitor: &mut V) -> T {
        visitor.visit_unary(self)
    }
}
