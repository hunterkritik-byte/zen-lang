# Zen

**Zen** is a small, readable programming language designed to grow into a practical general-purpose toolchain.

Zen programs use the `.zen` extension and run with the `zen` command.

## Why Zen?

Zen aims for a simple rule: **the language should be easy to read, easy to learn, and serious enough to build real software.**

The project is built from scratch in Rust, with a clean compiler architecture so the interpreter, type checker, formatter, package manager, and native compiler can evolve independently.

## Hello, Zen

```zen
fn main() {
    let name = "Zen"
    print("Hello from " + name)
}
```

Run it:

```bash
cargo run -- run examples/hello.zen
```

Check it without executing:

```bash
cargo run -- check examples/hello.zen
```

## Language today

The current development runtime supports:

- `.zen` source files
- functions and a `main` entry point
- local variables
- strings, integers, and booleans
- arithmetic: `+`, `-`, `*`, `/`
- string concatenation with `+`
- comparisons: `==`, `!=`, `<`, `<=`, `>`, `>=`
- `if` / `else`
- `while` loops
- function calls and return values
- line comments with `//`
- escaped strings (`\\n`, `\\t`, `\\r`, `\\"`, `\\\\`)
- checked integer arithmetic and division-by-zero errors

## Toolchain vision

The goal is a complete Zen development experience:

```text
zen run app.zen
zen check app.zen
zen build
zen test
zen fmt
zen lint
zen repl
zen doc
zen add package
```

Planned platform components include:

- a strong static type system with inference
- generics and expressive data types
- `Option` / `Result` error handling
- arrays, maps, sets, and pattern matching
- lexical scoping, closures, and first-class functions
- modules and imports
- a batteries-included standard library
- formatter, linter, test runner, and language server
- package manager and package registry
- C FFI
- bytecode VM and eventually native compilation
- high-quality diagnostics with source locations and actionable messages

## Project status

**Early development — 0.2.x.**

Zen is not yet a production compiler. Syntax and semantics can still change while the language foundation is being built. The project prioritizes correctness, tests, documentation, and a stable language specification before a 1.0 release.

## Roadmap

- [x] Lexer
- [x] Parser and AST
- [x] Variables and basic values
- [x] Functions and `main`
- [x] Arithmetic and comparisons
- [x] Control flow
- [x] Runtime error handling
- [ ] Source spans and compiler diagnostics
- [ ] Static type checker
- [ ] Arrays and indexing
- [ ] Maps and sets
- [ ] Closures
- [ ] Modules and imports
- [ ] `Option` / `Result`
- [ ] Standard library
- [ ] Formatter and linter
- [ ] `zen test` and REPL
- [ ] Package manager
- [ ] LSP / editor support
- [ ] Bytecode VM
- [ ] Native compiler backend
- [ ] Stable language specification
- [ ] 1.0 release

## Development

Requirements:

- Rust stable

Build and test:

```bash
cargo fmt --all -- --check
cargo test --all-targets
cargo build --release
```

## License

MIT
