use std::collections::HashMap;
use crate::ast::{Cmp, Expr, Stmt};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Key { Str(String), Num(i64), Bool(bool) }

#[derive(Clone, Debug, PartialEq)]
enum Value {
    Str(String),
    Num(i64),
    Bool(bool),
    Array(Vec<Value>),
    Map(HashMap<Key, Value>),
    Unit,
}

impl Value {
    fn truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Num(n) => *n != 0,
            Value::Str(s) => !s.is_empty(),
            Value::Array(v) => !v.is_empty(),
            Value::Map(v) => !v.is_empty(),
            Value::Unit => false,
        }
    }
}

type Functions = HashMap<String, (Vec<String>, Vec<Stmt>)>;

pub fn run(program: &[Stmt]) -> Result<(), String> {
    let mut funcs = Functions::new();
    for stmt in program {
        if let Stmt::Fn { name, params, body } = stmt {
            if funcs.insert(name.clone(), (params.clone(), body.clone())).is_some() {
                return Err(format!("duplicate function: {name}"));
            }
        }
    }
    let Some((params, body)) = funcs.get("main").cloned() else {
        return Err("entry point 'main' not found".into());
    };
    if !params.is_empty() { return Err("main must not have parameters".into()); }
    let mut env = HashMap::new();
    exec(&body, &mut env, &funcs)?;
    Ok(())
}

fn exec(body: &[Stmt], env: &mut HashMap<String, Value>, funcs: &Functions) -> Result<Option<Value>, String> {
    for stmt in body {
        match stmt {
            Stmt::Let(name, expr) => { let value = eval(expr, env, funcs)?; env.insert(name.clone(), value); }
            Stmt::Print(expr) => println!("{}", display(&eval(expr, env, funcs)?)),
            Stmt::Expr(expr) => { eval(expr, env, funcs)?; }
            Stmt::Fn { .. } => {}
            Stmt::If { condition, then_body, else_body } => {
                let branch = if eval(condition, env, funcs)?.truthy() { then_body } else { else_body };
                if let Some(value) = exec(branch, env, funcs)? { return Ok(Some(value)); }
            }
            Stmt::While { condition, body } => {
                while eval(condition, env, funcs)?.truthy() {
                    if let Some(value) = exec(body, env, funcs)? { return Ok(Some(value)); }
                }
            }
            Stmt::Return(expr) => return Ok(Some(match expr { Some(expr) => eval(expr, env, funcs)?, None => Value::Unit })),
        }
    }
    Ok(None)
}

fn eval(expr: &Expr, env: &HashMap<String, Value>, funcs: &Functions) -> Result<Value, String> {
    match expr {
        Expr::Str(s) => Ok(Value::Str(s.clone())),
        Expr::Num(n) => Ok(Value::Num(*n)),
        Expr::Bool(b) => Ok(Value::Bool(*b)),
        Expr::Array(items) => items.iter().map(|item| eval(item, env, funcs)).collect::<Result<Vec<_>, _>>().map(Value::Array),
        Expr::Map(entries) => {
            let mut map = HashMap::with_capacity(entries.len());
            for (key, value) in entries {
                let key = to_key(eval(key, env, funcs)?)?;
                let value = eval(value, env, funcs)?;
                map.insert(key, value);
            }
            Ok(Value::Map(map))
        }
        Expr::Index(target, index) => {
            let target = eval(target, env, funcs)?;
            let index = eval(index, env, funcs)?;
            match target {
                Value::Array(items) => {
                    let Value::Num(index) = index else { return Err("array index must be an integer".into()) };
                    if index < 0 { return Err(format!("array index out of bounds: {index}")); }
                    items.get(index as usize).cloned().ok_or_else(|| format!("array index out of bounds: {index}"))
                }
                Value::Map(map) => {
                    let key = to_key(index)?;
                    map.get(&key).cloned().ok_or_else(|| "map key not found".into())
                }
                value => Err(format!("cannot index {value:?}")),
            }
        }
        Expr::Var(name) => env.get(name).cloned().ok_or_else(|| format!("undefined variable: {name}")),
        Expr::Add(a, b) => {
            let x = eval(a, env, funcs)?; let y = eval(b, env, funcs)?;
            match (x, y) {
                (Value::Num(a), Value::Num(b)) => a.checked_add(b).map(Value::Num).ok_or("integer overflow".into()),
                (Value::Str(a), Value::Str(b)) => Ok(Value::Str(a + &b)),
                (a, b) => Err(format!("cannot add {a:?} and {b:?}")),
            }
        }
        Expr::Sub(a, b) => Ok(Value::Num(num(a, env, funcs)?.checked_sub(num(b, env, funcs)?).ok_or("integer overflow")?)),
        Expr::Mul(a, b) => Ok(Value::Num(num(a, env, funcs)?.checked_mul(num(b, env, funcs)?).ok_or("integer overflow")?)),
        Expr::Div(a, b) => {
            let y = num(b, env, funcs)?;
            if y == 0 { return Err("division by zero".into()); }
            Ok(Value::Num(num(a, env, funcs)?.checked_div(y).ok_or("integer overflow")?))
        }
        Expr::Compare(a, cmp, b) => { let x = eval(a, env, funcs)?; let y = eval(b, env, funcs)?; compare(&x, *cmp, &y) }
        Expr::Call(name, args) => {
            let (params, body) = funcs.get(name).cloned().ok_or_else(|| format!("undefined function: {name}"))?;
            if params.len() != args.len() { return Err(format!("function {name} expects {}, got {}", params.len(), args.len())); }
            let mut local = HashMap::new();
            for (param, arg) in params.iter().zip(args) { local.insert(param.clone(), eval(arg, env, funcs)?); }
            Ok(exec(&body, &mut local, funcs)?.unwrap_or(Value::Unit))
        }
    }
}

fn to_key(value: Value) -> Result<Key, String> {
    match value {
        Value::Str(s) => Ok(Key::Str(s)),
        Value::Num(n) => Ok(Key::Num(n)),
        Value::Bool(b) => Ok(Key::Bool(b)),
        _ => Err("map keys must be integers, strings, or booleans".into()),
    }
}

fn num(expr: &Expr, env: &HashMap<String, Value>, funcs: &Functions) -> Result<i64, String> {
    match eval(expr, env, funcs)? { Value::Num(n) => Ok(n), value => Err(format!("expected number, got {value:?}")) }
}

fn compare(a: &Value, cmp: Cmp, b: &Value) -> Result<Value, String> {
    let result = match (a, b) {
        (Value::Num(a), Value::Num(b)) => match cmp { Cmp::Eq => a == b, Cmp::Ne => a != b, Cmp::Lt => a < b, Cmp::Le => a <= b, Cmp::Gt => a > b, Cmp::Ge => a >= b },
        (Value::Str(a), Value::Str(b)) => match cmp { Cmp::Eq => a == b, Cmp::Ne => a != b, Cmp::Lt => a < b, Cmp::Le => a <= b, Cmp::Gt => a > b, Cmp::Ge => a >= b },
        (Value::Bool(a), Value::Bool(b)) => match cmp { Cmp::Eq => a == b, Cmp::Ne => a != b, _ => return Err("booleans only support == and !=".into()) },
        _ => return Err("values are not comparable".into()),
    };
    Ok(Value::Bool(result))
}

fn display(value: &Value) -> String {
    match value {
        Value::Str(s) => s.clone(), Value::Num(n) => n.to_string(), Value::Bool(b) => b.to_string(),
        Value::Array(items) => format!("[{}]", items.iter().map(display).collect::<Vec<_>>().join(", ")),
        Value::Map(map) => {
            let mut entries = map.iter().map(|(k, v)| format!("{}: {}", display_key(k), display(v))).collect::<Vec<_>>();
            entries.sort();
            format!("{{{}}}", entries.join(", "))
        }
        Value::Unit => "()".into(),
    }
}

fn display_key(key: &Key) -> String { match key { Key::Str(s) => format!("\"{s}\""), Key::Num(n) => n.to_string(), Key::Bool(b) => b.to_string() } }
