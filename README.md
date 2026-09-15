# Zen

Zen is a small, modern programming language with `.zen` source files. The project is being built from scratch with a focus on readable syntax, a tiny core, and tooling that can grow with the language.

## Quick start

Build the interpreter with Rust:

```bash
cargo build --release
```

Run a Zen program:

```bash
./target/release/zen run examples/hello.zen
```

Output:

```text
Hello from Zen
```

## Zen example

```zen
fn main() {
    let name = "Zen"
    print("Hello from " + name)
}
```

## Roadmap

- [x] `.zen` source files
- [x] Lexer
- [x] Parser and AST
- [x] Variables and strings
- [x] Functions and `main`
- [x] Basic `print`
- [ ] Numbers and arithmetic
- [ ] Booleans and comparisons
- [ ] `if` / `else`
- [ ] Loops
- [ ] Arrays and maps
- [ ] Modules and imports
- [ ] Formatter
- [ ] REPL
- [ ] Package manager
- [ ] Python package/distribution tooling
- [ ] Native compiler backend

## Project status

Early development (0.1.0). Syntax and semantics may change.

## License

MIT
