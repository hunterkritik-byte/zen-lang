use std::collections::HashMap;
use crate::ast::{Cmp, Expr, Stmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type { Int, String, Bool, Unit, Unknown }

pub fn check(program: &[Stmt]) -> Result<(), String> {
    let mut functions = HashMap::new();
    for stmt in program { if let Stmt::Fn { name, params, .. } = stmt { functions.insert(name.clone(), params.len()); } }
    for stmt in program {
        if let Stmt::Fn { name, params, body } = stmt {
            let mut env = HashMap::new();
            for p in params { env.insert(p.clone(), Type::Unknown); }
            check_block(body, &mut env, &functions).map_err(|e| format!("in function {name}: {e}"))?;
        }
    }
    Ok(())
}

fn check_block(body: &[Stmt], env: &mut HashMap<String, Type>, funcs: &HashMap<String, usize>) -> Result<(), String> {
    for stmt in body {
        match stmt {
            Stmt::Let(n, e) => { env.insert(n.clone(), infer(e, env, funcs)?); }
            Stmt::Print(e) | Stmt::Expr(e) => { infer(e, env, funcs)?; }
            Stmt::Fn { .. } => {}
            Stmt::If { condition, then_body, else_body } => { require_bool(infer(condition, env, funcs)?)?; let mut a=env.clone(); let mut b=env.clone(); check_block(then_body,&mut a,funcs)?; check_block(else_body,&mut b,funcs)?; }
            Stmt::While { condition, body } => { require_bool(infer(condition, env, funcs)?)?; let mut local=env.clone(); check_block(body,&mut local,funcs)?; }
            Stmt::Return(e) => { if let Some(e)=e { infer(e,env,funcs)?; } }
        }
    }
    Ok(())
}

fn infer(e:&Expr, env:&HashMap<String,Type>, funcs:&HashMap<String,usize>)->Result<Type,String>{
    match e {
        Expr::Str(_)=>Ok(Type::String), Expr::Num(_)=>Ok(Type::Int), Expr::Bool(_)=>Ok(Type::Bool),
        Expr::Var(n)=>env.get(n).cloned().ok_or_else(||format!("undefined variable '{n}'")),
        Expr::Add(a,b)=>{let x=infer(a,env,funcs)?;let y=infer(b,env,funcs)?;if x==Type::Unknown||y==Type::Unknown||x==y {if x==Type::Int||x==Type::String||x==Type::Unknown{Ok(x)}else{Err(format!("cannot add {x:?} and {y:?}"))}}else{Err(format!("cannot add {x:?} and {y:?}"))}},
        Expr::Sub(a,b)|Expr::Mul(a,b)|Expr::Div(a,b)=>{require_int(infer(a,env,funcs)?)?;require_int(infer(b,env,funcs)?)?;Ok(Type::Int)},
        Expr::Compare(a,c,b)=>{let x=infer(a,env,funcs)?;let y=infer(b,env,funcs)?;if x!=Type::Unknown&&y!=Type::Unknown&&x!=y{return Err(format!("cannot compare {x:?} and {y:?}"));}if matches!(c,Cmp::Lt|Cmp::Le|Cmp::Gt|Cmp::Ge)&&!matches!(x,Type::Int|Type::String|Type::Unknown){return Err("ordered comparison requires integers or strings".into())}Ok(Type::Bool)},
        Expr::Call(n,args)=>{let expected=funcs.get(n).ok_or_else(||format!("undefined function '{n}'"))?;if *expected!=args.len(){return Err(format!("function {n} expects {expected} arguments, got {}",args.len()))}for a in args{infer(a,env,funcs)?;}Ok(Type::Unknown)}
    }
}
fn require_int(t:Type)->Result<(),String>{if matches!(t,Type::Int|Type::Unknown){Ok(())}else{Err(format!("expected integer, got {t:?}"))}}
fn require_bool(t:Type)->Result<(),String>{if matches!(t,Type::Bool|Type::Unknown){Ok(())}else{Err(format!("condition must be boolean, got {t:?}"))}}
