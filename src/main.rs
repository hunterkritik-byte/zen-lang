use std::{env, fs, process};
use zen_lang::{check, lex, parse, run};

const VERSION: &str = "0.2.0";

fn usage() {
    println!("Zen {VERSION}\n\nUsage:\n  zen run <file.zen>\n  zen check <file.zen>\n  zen version\n  zen help");
}

fn load(path: &str) -> Result<Vec<zen_lang::ast::Stmt>, String> {
    let src = fs::read_to_string(path).map_err(|e| format!("cannot read {path}: {e}"))?;
    let tokens = lex(&src).map_err(|e| format!("{path}: {e}"))?;
    parse(tokens).map_err(|e| format!("{path}: {e}"))
}

fn validate(path: &str) -> Result<Vec<zen_lang::ast::Stmt>, String> {
    let program = load(path)?;
    check(&program).map_err(|e| format!("{path}: type error: {e}"))?;
    Ok(program)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("version") | Some("--version") | Some("-V") => println!("Zen {VERSION}"),
        Some("help") | Some("--help") | Some("-h") => usage(),
        Some("check") => {
            let Some(path) = args.get(2) else { eprintln!("error: missing input file"); process::exit(2) };
            match validate(path) { Ok(_) => println!("OK: {path}"), Err(e) => { eprintln!("error: {e}"); process::exit(1) } }
        }
        Some("run") => {
            let Some(path) = args.get(2) else { eprintln!("error: missing input file"); process::exit(2) };
            match validate(path).and_then(|program| run(&program)) { Ok(_) => {}, Err(e) => { eprintln!("error: {e}"); process::exit(1) } }
        }
        _ => { usage(); process::exit(2) }
    }
}
