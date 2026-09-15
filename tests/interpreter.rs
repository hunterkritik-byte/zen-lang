use zen_lang::{check, lex, parse, run};

fn run_source(source: &str) -> Result<(), String> { run(&parse(lex(source)?)?) }
fn check_source(source: &str) -> Result<(), String> { check(&parse(lex(source)?)?) }

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

#[test]
fn arrays_support_indexing() {
    run_source(r#"
        fn main() {
            let items = [10, 20, 30]
            print(items[1])
        }
    "#).unwrap();
}

#[test]
fn nested_array_indexing_works() {
    run_source(r#"
        fn main() {
            let matrix = [[1, 2], [3, 4]]
            print(matrix[1][0])
        }
    "#).unwrap();
}

#[test]
fn array_index_errors_are_safe() {
    let err = run_source(r#"
        fn main() {
            let items = [1, 2]
            print(items[2])
        }
    "#).unwrap_err();
    assert!(err.contains("array index out of bounds"));
}

#[test]
fn maps_support_string_keys_and_indexing() {
    run_source(r#"
        fn main() {
            let user = {"name": "Zen", "age": 2}
            print(user["name"])
            print(user["age"])
        }
    "#).unwrap();
}

#[test]
fn maps_support_integer_and_boolean_keys() {
    run_source(r#"
        fn main() {
            let values = {1: "one", true: "yes"}
            print(values[1])
            print(values[true])
        }
    "#).unwrap();
}

#[test]
fn missing_map_key_is_rejected() {
    let err = run_source(r#"
        fn main() {
            let user = {"name": "Zen"}
            print(user["missing"])
        }
    "#).unwrap_err();
    assert!(err.contains("map key not found"));
}

#[test]
fn checker_accepts_collections() {
    check_source(r#"
        fn main() {
            let items = [10, 20, 30]
            let user = {"name": "Zen", "age": 2}
            print(items[0] + user["age"])
        }
    "#).unwrap();
}

#[test]
fn checker_rejects_bad_array_index() {
    let err = check_source(r#"
        fn main() {
            let items = [1, 2]
            print(items["bad"])
        }
    "#).unwrap_err();
    assert!(err.contains("expected integer"));
}
