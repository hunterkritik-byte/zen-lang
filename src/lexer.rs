#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Fn, Let, Print, True, False, If, Else, While, Return,
    Ident(String), String(String), Number(i64),
    LBrace, RBrace, LParen, RParen, Eq, Plus, Minus, Star, Slash,
    EqEq, NotEq, Lt, LtEq, Gt, GtEq, Comma, Semi, Eof,
}

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut out = Vec::new(); let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            c if c.is_whitespace() => { chars.next(); }
            '/' => { chars.next(); if chars.peek()==Some(&'/') { while let Some(x)=chars.next(){if x=='\n'{break}} } else {out.push(Token::Slash)} }
            '{'=>{chars.next();out.push(Token::LBrace)} '}' =>{chars.next();out.push(Token::RBrace)}
            '('=>{chars.next();out.push(Token::LParen)} ')'=>{chars.next();out.push(Token::RParen)}
            ','=>{chars.next();out.push(Token::Comma)} ';'=>{chars.next();out.push(Token::Semi)}
            '+'=>{chars.next();out.push(Token::Plus)} '-'=>{chars.next();out.push(Token::Minus)}
            '*'=>{chars.next();out.push(Token::Star)}
            '='=>{chars.next();if chars.peek()==Some(&'='){chars.next();out.push(Token::EqEq)}else{out.push(Token::Eq)}}
            '!'=>{chars.next();if chars.peek()==Some(&'='){chars.next();out.push(Token::NotEq)}else{return Err("unexpected '!'".into())}}
            '<'=>{chars.next();if chars.peek()==Some(&'='){chars.next();out.push(Token::LtEq)}else{out.push(Token::Lt)}}
            '>'=>{chars.next();if chars.peek()==Some(&'='){chars.next();out.push(Token::GtEq)}else{out.push(Token::Gt)}}
            '"'=>{chars.next();let mut s=String::new();let mut closed=false;while let Some(x)=chars.next(){if x=='"'{closed=true;break} if x=='\\'{match chars.next(){Some('n')=>s.push('\n'),Some('t')=>s.push('\t'),Some('r')=>s.push('\r'),Some('"')=>s.push('"'),Some('\\')=>s.push('\\'),Some(x)=>return Err(format!("unknown escape: \\{x}")),None=>return Err("unterminated string".into())}}else{s.push(x)}}if !closed{return Err("unterminated string".into())}out.push(Token::String(s))}
            c if c.is_ascii_digit()=>{let mut n=String::new();while chars.peek().is_some_and(|x|x.is_ascii_digit()){n.push(chars.next().unwrap())}out.push(Token::Number(n.parse().map_err(|_|"invalid number")?))}
            c if c.is_ascii_alphabetic()||c=='_'=>{let mut s=String::new();while chars.peek().is_some_and(|x|x.is_ascii_alphanumeric()||*x=='_'){s.push(chars.next().unwrap())}out.push(match s.as_str(){"fn"=>Token::Fn,"let"=>Token::Let,"print"=>Token::Print,"true"=>Token::True,"false"=>Token::False,"if"=>Token::If,"else"=>Token::Else,"while"=>Token::While,"return"=>Token::Return,_=>Token::Ident(s)})}
            _=>return Err(format!("unexpected character: {c}")),
        }
    } out.push(Token::Eof);Ok(out)
}
