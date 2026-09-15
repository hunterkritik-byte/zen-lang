use crate::ast::{Cmp, Expr, Stmt};
use crate::lexer::Token;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, String> { Parser { tokens, pos: 0 }.program() }

struct Parser { tokens: Vec<Token>, pos: usize }

impl Parser {
    fn peek(&self) -> &Token { &self.tokens[self.pos] }
    fn next(&mut self) -> Token { let t = self.tokens[self.pos].clone(); self.pos += 1; t }
    fn eat(&mut self, t: &Token) -> bool { if self.peek() == t { self.next(); true } else { false } }
    fn expect(&mut self, t: Token) -> Result<(), String> {
        let got = self.next();
        if got == t { Ok(()) } else { Err(format!("expected {t:?}, got {got:?}")) }
    }
    fn program(&mut self) -> Result<Vec<Stmt>, String> {
        let mut v = vec![];
        while *self.peek() != Token::Eof { v.push(self.stmt()?); }
        Ok(v)
    }
    fn block(&mut self) -> Result<Vec<Stmt>, String> {
        self.expect(Token::LBrace)?;
        let mut v = vec![];
        while *self.peek() != Token::RBrace && *self.peek() != Token::Eof { v.push(self.stmt()?); }
        self.expect(Token::RBrace)?;
        Ok(v)
    }
    fn stmt(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Token::Let => { self.next(); let n = self.ident()?; self.expect(Token::Eq)?; let e = self.expr()?; self.eat(&Token::Semi); Ok(Stmt::Let(n, e)) }
            Token::Print => { self.next(); self.expect(Token::LParen)?; let e = self.expr()?; self.expect(Token::RParen)?; self.eat(&Token::Semi); Ok(Stmt::Print(e)) }
            Token::Fn => self.function(),
            Token::If => { self.next(); let c = self.expr()?; let a = self.block()?; let b = if self.eat(&Token::Else) { self.block()? } else { vec![] }; Ok(Stmt::If { condition: c, then_body: a, else_body: b }) }
            Token::While => { self.next(); let c = self.expr()?; let b = self.block()?; Ok(Stmt::While { condition: c, body: b }) }
            Token::Return => { self.next(); let e = if *self.peek() == Token::Semi || *self.peek() == Token::RBrace { None } else { Some(self.expr()?) }; self.eat(&Token::Semi); Ok(Stmt::Return(e)) }
            _ => { let e = self.expr()?; self.eat(&Token::Semi); Ok(Stmt::Expr(e)) }
        }
    }
    fn ident(&mut self) -> Result<String, String> { match self.next() { Token::Ident(s) => Ok(s), x => Err(format!("expected identifier, got {x:?}")) } }
    fn function(&mut self) -> Result<Stmt, String> {
        self.next(); let name = self.ident()?; self.expect(Token::LParen)?;
        let mut p = vec![];
        if *self.peek() != Token::RParen { loop { p.push(self.ident()?); if !self.eat(&Token::Comma) { break; } } }
        self.expect(Token::RParen)?;
        Ok(Stmt::Fn { name, params: p, body: self.block()? })
    }
    fn expr(&mut self) -> Result<Expr, String> { self.compare() }
    fn compare(&mut self) -> Result<Expr, String> {
        let mut e = self.add()?;
        let c = match self.peek() { Token::EqEq => Some(Cmp::Eq), Token::NotEq => Some(Cmp::Ne), Token::Lt => Some(Cmp::Lt), Token::LtEq => Some(Cmp::Le), Token::Gt => Some(Cmp::Gt), Token::GtEq => Some(Cmp::Ge), _ => None };
        if let Some(c) = c { self.next(); e = Expr::Compare(Box::new(e), c, Box::new(self.add()?)); }
        Ok(e)
    }
    fn add(&mut self) -> Result<Expr, String> {
        let mut e = self.mul()?;
        loop { if self.eat(&Token::Plus) { e = Expr::Add(Box::new(e), Box::new(self.mul()?)); } else if self.eat(&Token::Minus) { e = Expr::Sub(Box::new(e), Box::new(self.mul()?)); } else { break; } }
        Ok(e)
    }
    fn mul(&mut self) -> Result<Expr, String> {
        let mut e = self.postfix()?;
        loop { if self.eat(&Token::Star) { e = Expr::Mul(Box::new(e), Box::new(self.postfix()?)); } else if self.eat(&Token::Slash) { e = Expr::Div(Box::new(e), Box::new(self.postfix()?)); } else { break; } }
        Ok(e)
    }
    fn postfix(&mut self) -> Result<Expr, String> {
        let mut e = self.primary()?;
        while self.eat(&Token::LBracket) {
            let index = self.expr()?;
            self.expect(Token::RBracket)?;
            e = Expr::Index(Box::new(e), Box::new(index));
        }
        Ok(e)
    }
    fn primary(&mut self) -> Result<Expr, String> {
        match self.next() {
            Token::String(s) => Ok(Expr::Str(s)),
            Token::Number(n) => Ok(Expr::Num(n)),
            Token::True => Ok(Expr::Bool(true)),
            Token::False => Ok(Expr::Bool(false)),
            Token::Ident(n) => {
                if self.eat(&Token::LParen) {
                    let mut a = vec![];
                    if *self.peek() != Token::RParen { loop { a.push(self.expr()?); if !self.eat(&Token::Comma) { break; } } }
                    self.expect(Token::RParen)?;
                    Ok(Expr::Call(n, a))
                } else { Ok(Expr::Var(n)) }
            }
            Token::LParen => { let e = self.expr()?; self.expect(Token::RParen)?; Ok(e) }
            Token::LBracket => {
                let mut items = vec![];
                if *self.peek() != Token::RBracket { loop { items.push(self.expr()?); if !self.eat(&Token::Comma) { break; } } }
                self.expect(Token::RBracket)?;
                Ok(Expr::Array(items))
            }
            Token::LBrace => {
                let mut entries = vec![];
                if *self.peek() != Token::RBrace {
                    loop {
                        let key = self.expr()?;
                        self.expect(Token::Colon)?;
                        let value = self.expr()?;
                        entries.push((key, value));
                        if !self.eat(&Token::Comma) { break; }
                    }
                }
                self.expect(Token::RBrace)?;
                Ok(Expr::Map(entries))
            }
            x => Err(format!("expected expression, got {x:?}")),
        }
    }
}
