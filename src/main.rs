use std::{env, fs, process};

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Fn, Let, Print, Ident(String), String(String), Number(i64),
    LBrace, RBrace, LParen, RParen, Eq, Plus, Comma, Semi, Eof,
}

fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut out = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            c if c.is_whitespace() => { chars.next(); }
            '{' => { chars.next(); out.push(Token::LBrace); }
            '}' => { chars.next(); out.push(Token::RBrace); }
            '(' => { chars.next(); out.push(Token::LParen); }
            ')' => { chars.next(); out.push(Token::RParen); }
            '=' => { chars.next(); out.push(Token::Eq); }
            '+' => { chars.next(); out.push(Token::Plus); }
            ',' => { chars.next(); out.push(Token::Comma); }
            ';' => { chars.next(); out.push(Token::Semi); }
            '"' => {
                chars.next();
                let mut s = String::new();
                while let Some(c) = chars.next() {
                    if c == '"' { break; }
                    if c == '\\' {
                        match chars.next() {
                            Some('n') => s.push('\n'), Some('t') => s.push('\t'),
                            Some('"') => s.push('"'), Some('\\') => s.push('\\'),
                            Some(x) => return Err(format!("unknown escape: \\{x}")),
                            None => return Err("unterminated string".into()),
                        }
                    } else { s.push(c); }
                }
                out.push(Token::String(s));
            }
            c if c.is_ascii_digit() => {
                let mut n = String::new();
                while chars.peek().is_some_and(|x| x.is_ascii_digit()) { n.push(chars.next().unwrap()); }
                out.push(Token::Number(n.parse().map_err(|_| "invalid number")?));
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut s = String::new();
                while chars.peek().is_some_and(|x| x.is_ascii_alphanumeric() || *x == '_') { s.push(chars.next().unwrap()); }
                out.push(match s.as_str() {
                    "fn" => Token::Fn, "let" => Token::Let, "print" => Token::Print,
                    _ => Token::Ident(s),
                });
            }
            _ => return Err(format!("unexpected character: {c}")),
        }
    }
    out.push(Token::Eof);
    Ok(out)
}

struct Parser { tokens: Vec<Token>, pos: usize }
impl Parser {
    fn peek(&self) -> &Token { &self.tokens[self.pos] }
    fn next(&mut self) -> Token { let t = self.tokens[self.pos].clone(); self.pos += 1; t }
    fn expect(&mut self, wanted: Token) -> Result<(), String> {
        let got = self.next(); if got == wanted { Ok(()) } else { Err(format!("expected {wanted:?}, got {got:?}")) }
    }
    fn program(&mut self) -> Result<Vec<Stmt>, String> {
        let mut body = Vec::new();
        while *self.peek() != Token::Eof { body.push(self.stmt()?); }
        Ok(body)
    }
    fn stmt(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Token::Let => { self.next(); let name = match self.next() { Token::Ident(s) => s, x => return Err(format!("expected name, got {x:?}")) }; self.expect(Token::Eq)?; let e = self.expr()?; if *self.peek()==Token::Semi { self.next(); } Ok(Stmt::Let(name,e)) }
            Token::Print => { self.next(); self.expect(Token::LParen)?; let e=self.expr()?; self.expect(Token::RParen)?; if *self.peek()==Token::Semi { self.next(); } Ok(Stmt::Print(e)) }
            Token::Fn => self.function(),
            _ => { let e=self.expr()?; if *self.peek()==Token::Semi {self.next();} Ok(Stmt::Expr(e)) }
        }
    }
    fn function(&mut self) -> Result<Stmt,String> {
        self.expect(Token::Fn)?;
        let name=match self.next(){Token::Ident(s)=>s,x=>return Err(format!("expected function name, got {x:?}"))};
        self.expect(Token::LParen)?; self.expect(Token::RParen)?; self.expect(Token::LBrace)?;
        let mut body=Vec::new(); while *self.peek()!=Token::RBrace { body.push(self.stmt()?); }
        self.expect(Token::RBrace)?; Ok(Stmt::Fn(name,body))
    }
    fn expr(&mut self) -> Result<Expr,String> {
        let mut left=self.primary()?;
        while *self.peek()==Token::Plus { self.next(); left=Expr::Add(Box::new(left),Box::new(self.primary()?)); }
        Ok(left)
    }
    fn primary(&mut self)->Result<Expr,String>{match self.next(){Token::String(s)=>Ok(Expr::Str(s)),Token::Number(n)=>Ok(Expr::Num(n)),Token::Ident(s)=>Ok(Expr::Var(s)),x=>Err(format!("expected expression, got {x:?}"))}}
}

#[derive(Debug)]
enum Stmt { Let(String,Expr), Print(Expr), Expr(Expr), Fn(String,Vec<Stmt>) }
#[derive(Debug)]
enum Expr { Str(String), Num(i64), Var(String), Add(Box<Expr>,Box<Expr>) }

use std::collections::HashMap;
fn value(e:&Expr, env:&HashMap<String,String>)->Result<String,String>{match e{Expr::Str(s)=>Ok(s.clone()),Expr::Num(n)=>Ok(n.to_string()),Expr::Var(n)=>env.get(n).cloned().ok_or_else(||format!("undefined variable: {n}")),Expr::Add(a,b)=>Ok(format!("{}{}",value(a,env)?,value(b,env)?))}}
fn run(body:&[Stmt], env:&mut HashMap<String,String>)->Result<(),String>{for s in body{match s{Stmt::Let(n,e)=>{env.insert(n.clone(),value(e,env)?);},Stmt::Print(e)=>println!("{}",value(e,env)?),Stmt::Expr(e)=>{value(e,env)?;},Stmt::Fn(_,_)=>{}}}if let Some(Stmt::Fn(_,b))=body.iter().find(|s|matches!(s,Stmt::Fn(n,_) if n=="main")){run(b,env)?;}Ok(())}

fn main(){
    let args:Vec<String>=env::args().collect();
    if args.len()!=3 || args[1]!="run" { eprintln!("Zen 0.1.0\nUsage: zen run <file.zen>"); process::exit(2); }
    let src=match fs::read_to_string(&args[2]){Ok(s)=>s,Err(e)=>{eprintln!("error: {e}");process::exit(1)}};
    match lex(&src).and_then(|t|{let mut p=Parser{tokens:t,pos:0};p.program()}).and_then(|p|run(&p,&mut HashMap::new())){Ok(())=>{},Err(e)=>{eprintln!("error: {e}");process::exit(1)}}
}
