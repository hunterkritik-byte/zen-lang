use zen_lang::{lex, parse, run};

fn run_source(source: &str) -> Result<(), String> {
    run(&parse(lex(source)?)?)
}

#[test]
fn hello_program_parses_and_runs() {
    run_source(r#"
        fn main() {
            let name = "Zen"
            print("Hello from " + name)
        }
    "#).unwrap();
}

#[test]
fn arithmetic_precedence_is_correct() {
    run_source(r#"
        fn main() {
            let value = 2 + 3 * 4
            print(value)
        }
    "#).unwrap();
}

#[test]
fn division_by_zero_is_rejected() {
    let err = run_source(r#"
        fn main() {
            print(10 / 0)
        }
    "#).unwrap_err();
    assert!(err.contains("division by zero"));
}

#[test]
fn functions_can_return_values() {
    run_source(r#"
        fn double(x) {
            return x * 2
        }
        fn main() {
            let value = double(21)
            print(value)
        }
    "#).unwrap();
}

#[test]
fn control_flow_parses_and_runs() {
    run_source(r#"
        fn main() {
            let x = 1
            while x < 3 {
                print(x)
                let x = x + 1
            }
            if x == 3 {
                print("done")
            } else {
                print("bad")
            }
        }
    "#).unwrap();
}
