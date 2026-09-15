#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Expr),
    Print(Expr),
    Expr(Expr),
    Fn { name: String, params: Vec<String>, body: Vec<Stmt> },
    If { condition: Expr, then_body: Vec<Stmt>, else_body: Vec<Stmt> },
    While { condition: Expr, body: Vec<Stmt> },
    Return(Option<Expr>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Str(String),
    Num(i64),
    Bool(bool),
    Var(String),
    Array(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
    Index(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Compare(Box<Expr>, Cmp, Box<Expr>),
    Call(String, Vec<Expr>),
    // NEW: Closure support
    Closure { params: Vec<String>, body: Vec<Stmt>, captures: Vec<String> },
    ClosureCall(Box<Expr>, Vec<Expr>),
}

#[derive(Debug, Clone, Copy)]
pub enum Cmp { Eq, Ne, Lt, Le, Gt, Ge }
