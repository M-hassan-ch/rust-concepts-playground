# Rust learning lab

A hands-on Rust workspace: small, runnable programs that explore language fundamentals, ownership, and concurrency—from `println!` to Tokio async.

![Rust](https://img.shields.io/badge/rust-2024%20edition-orange?logo=rust&logoColor=white)
![Purpose](https://img.shields.io/badge/purpose-personal%20learning-8A2BE2)


## What’s inside

| Binary | Topic |
|--------|--------|
| `print` | Basic output |
| `loop` | Loops and control flow |
| `match` | Pattern matching |
| `enum` | Enumerations |
| `struct` | Structs and data layout |
| `mods` | Modules and organization |
| `borrow` | References and borrowing |
| `lifetime` | Lifetimes |
| `trait` | Traits |
| `question_operator` | The `?` operator and error flow |
| `thread` | `std::thread`, `join`, scoped threads |
| `channel` | `mpsc` channels, `recv` / `try_recv` |
| `async` | Async tasks with **Tokio** (`join!`, `sleep`, `.await`) |

Dependencies are minimal on purpose: only [**Tokio**](https://tokio.rs/) is pulled in for the async examples.

## Requirements

- [Rust toolchain](https://www.rust-lang.org/tools/install) (stable or nightly, depending on your `edition = "2024"` setup)
- `cargo` (ships with Rust)

## Run a sample

From the repo root:

```bash
cargo run --bin print
cargo run --bin thread
cargo run --bin channel
cargo run --bin async
```

## Project layout

```
.
├── Cargo.toml          # Package manifest + Tokio dependency
├── README.md
└── src/bin/            # One `main` per topic
    ├── async.rs
    ├── channel.rs
    └── ...
```

## Learning path (suggested order)

1. **Basics** — `print`, `loop`, `match`, `enum`, `struct`  
2. **Structure** — `mods`  
3. **Ownership** — `borrow`, `lifetime`, `trait`, `question_operator`  
4. **Concurrency** — `thread`, `channel`, `async`

## Contributing

This is a personal learning repo. If you fork it for your own notes, swap the badge links and add whatever experiments help you.

## License

No license file is included yet—treat this as personal notes. If you open-source it, add a `LICENSE` (for example MIT or Apache-2.0) so others know the terms.

---
