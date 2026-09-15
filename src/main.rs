use std::{env,fs,process};
use zen_lang::{lex,parse,run};
const VERSION:&str="0.2.0";
fn usage(){println!("Zen {VERSION}\n\nUsage:\n  zen run <file.zen>\n  zen check <file.zen>\n  zen version\n  zen help");}
fn load(path:&str)->Result<Vec<zen_lang::ast::Stmt>,String>{let src=fs::read_to_string(path).map_err(|e|format!("cannot read {path}: {e}"))?;parse(lex(&src)?)}
fn main(){let a:Vec<String>=env::args().collect();match a.get(1).map(String::as_str){Some("version")|Some("--version")|Some("-V")=>println!("Zen {VERSION}"),Some("help")|Some("--help")|Some("-h")=>usage(),Some("check")=>{let Some(p)=a.get(2)else{eprintln!("error: missing input file");process::exit(2)};match load(p){Ok(_)=>println!("OK: {p}"),Err(e)=>{eprintln!("error: {e}");process::exit(1)}}},Some("run")=>{let Some(p)=a.get(2)else{eprintln!("error: missing input file");process::exit(2)};match load(p).and_then(|p|run(&p)){Ok(_)=>{},Err(e)=>{eprintln!("error: {e}");process::exit(1)}}},_=>{usage();process::exit(2)}}}
