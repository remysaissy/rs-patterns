
use std:Vec;

// Visitor pattern that evaluate arithmetic expressions.
pub enum Stmt {
    Expr(expr),
    Let(Name, Expr),
}
impl Stmt {
    pub fn walk(visitor: &mut Visitor, e: &Stmt) {
        match *e {
            Stmt::Expr(ref e) => visitor.visit_expr(e),
            Stmt::Let(..) => unimplemented!(),
        }
    }
}

pub struct Name {
    value: String,
}
impl Name {
    pub fn walk(visitor: &mut Visitor, e: &Name) {
        unimplemented!();
    }
}

pub enum Expr {
    IntLit(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
}
impl Expr {
    pub fn walk(visitor: &mut Visitor, e: &Expr) {
        match *e {
            Expr::IntLit(_) => {},
            Expr::Add(ref lhs, ref rhs) => {
                visitor.visit_expr(lhs);
                visitor.visit_expr(rhs);
            }
            Expr::Sub(ref lhs, ref rhs) => {
                visitor.visit_expr(lhs);
                visitor.visit_expr(rhs);
            }
        }
    }
}

// Visitor pattern abstraction.
pub trait Visitor<T> {
    fn visit_name(&mut self, n: &Name) -> T;
    fn visit_stmt(&mut self, s: &Stmt) -> T;
    fn visit_expr(&mut self, e: &Expr) -> T;
}


// An example concrete implementation - walks the elements interpreting it as code.
struct Interpreter;
impl Visitor<i64> for Interpreter {
    fn visit_name(&mut self, n: &Name) -> i64 { panic!() }
    fn visit_stmt(&mut self, s: &Stmt) -> i64 {
        match *s {
            Stmt::Expr(ref e) => self.visit_expr(e),
            Stmt::Let(..) => unimplemented!(),
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> i64 {
        match *e {
            Expr::IntLit(n) => n,
            Expr::Add(ref lhs, ref rhs) => self.visit_expr(lhs) + self.visit_expr(rhs),
            Expr::Sub(ref lhs, ref rhs) => self.visit_expr(lhs) - self.visit_expr(rhs),
        }
    }
}

#[cfg(test)]
mod tests {
    
    // #[test]
    fn test_visit() {
        unimplemented!()
        // let v = DocumentVisitor::new();
        // let d = Document::new();
        // let e = WordElement::new("Hello");        
        // d.add_element.push(e);
        // let e = CommaElement::new();
        // d.add_element.push(e);
        // let e = WordElement::new("World");
        // d.add_element.push(e);
        // d.accept(v);
    }
}