use zen_lang::{lex,parse};

#[test]
fn parses_core_language(){
    let src=r#"fn add(a,b){ return a+b } fn main(){ let x=add(2,3); if x==5 { print("ok") } }"#;
    let program=parse(lex(src).unwrap()).unwrap();
    assert!(program.len()==2);
}

#[test]
fn rejects_unterminated_string(){
    assert!(lex("fn main(){ print(\"oops) }").is_err());
}
