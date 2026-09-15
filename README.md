# Zen

**Zen** is a small, modern programming language designed around readable syntax and a deliberately simple core. Zen source files use the `.zen` extension and the command-line tool is `zen`.

> Status: **early development — 0.2.0**. The language is experimental and syntax may change before 1.0.

## Features

- `.zen` source files
- Lexer and parser with a typed AST
- Variables and literals: strings, integers, booleans
- Arithmetic: `+`, `-`, `*`, `/`
- Comparisons: `==`, `!=`, `<`, `<=`, `>`, `>=`
- `if` / `else` and `while`
- Functions with parameters and `return`
- Built-in `print(...)`
- A standalone `zen` CLI
- `zen check` syntax validation
- Rust test suite and continuous integration

## Quick start

### Build

```bash
cargo build --release
```

### Run

```bash
./target/release/zen run examples/hello.zen
```

### Check without running

```bash
./target/release/zen check examples/hello.zen
```

### CLI help

```bash
./target/release/zen help
```

## Example

```zen
fn add(a, b) {
    return a + b
}

fn main() {
    let result = add(20, 22)

    if result == 42 {
        print("The answer is " + "42")
    } else {
        print("unexpected result")
    }
}
```

## Architecture

```text
.zen source
    ↓
 Lexer
    ↓
 Tokens
    ↓
 Parser
    ↓
 AST
    ↓
 Runtime
    ↓
 Program output
```

The compiler is intentionally split into public modules so the front end and runtime can later be reused by tooling, a formatter, REPL, language server, and package ecosystem.

## Development

Run the full local validation suite:

```bash
cargo fmt --all -- --check
cargo test --all-targets
cargo build --release
```

CI performs the same checks on pushes and pull requests.

## Roadmap to 1.0

- [x] Core lexer/parser/runtime
- [x] Functions, control flow, arithmetic and comparisons
- [x] CLI `run`, `check`, `help`, `version`
- [ ] Arrays, maps and indexing
- [ ] Modules and imports
- [ ] Stronger type system and diagnostics
- [ ] Standard library
- [ ] Formatter
- [ ] REPL
- [ ] Language server / editor support
- [ ] Package manager
- [ ] Cross-platform release binaries
- [ ] Python/PyPI distribution of the Zen toolchain
- [ ] Native compiler backend
- [ ] 1.0 language specification and compatibility guarantees

## Releases and PyPI

PyPI distribution will come **after the language and CLI stabilize**. The planned Python package will act as distribution/tooling around the Zen compiler and CLI; Zen itself is not a Python language implementation.

## License

MIT
