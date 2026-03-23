# Rust learning lab

A hands-on Rust workspace: small programs in `src/bin/` for language fundamentals and concurrency, plus a **library + default binary** that practices real crate layout and modules.

![Rust](https://img.shields.io/badge/rust-2024%20edition-orange?logo=rust&logoColor=white)
![Purpose](https://img.shields.io/badge/purpose-personal%20learning-8A2BE2)

## What’s inside

### Default package: library + binary

The crate root is both a **library** (`src/lib.rs`) and a **binary** (`src/main.rs`). The binary imports the library API by package name (`learning`, from `Cargo.toml`).

| Piece | Role |
|--------|------|
| `src/lib.rs` | Crate root: declares top-level modules and the public surface of the library. |
| `src/main.rs` | Default `main`; calls into `learning::auth::login` with `Credentials`. |
| `src/databae.rs` | `databae` module: `Status`, `connect_to_db`, `get_user` (placeholder DB flow). |
| `src/auth.rs` | `auth` module: `login`, `authenticate` stub; pulls DB helpers from `databae`. |
| `src/auth/models.rs` | `auth::models`: `Credentials` (`username`, `password`). |

This layout grew in three steps: **library vs binary** → **modules inside `lib.rs`** → **modules split across files** (`auth` / `auth/models` / `databae`).

### Topic binaries (`src/bin/`)

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

Dependencies stay small: only [**Tokio**](https://tokio.rs/) for the async examples.

## Requirements

- [Rust toolchain](https://www.rust-lang.org/tools/install) matching `edition = "2024"` in `Cargo.toml`
- `cargo` (ships with Rust)

## Run

**Default binary** (library demo):

```bash
cargo run
```

**Topic examples:**

```bash
cargo run --bin print
cargo run --bin thread
cargo run --bin channel
cargo run --bin async
```

## Project layout

```
.
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs              # Crate root: pub mod auth, pub mod databae
│   ├── main.rs             # Default binary → learning::auth::…
│   ├── auth.rs
│   ├── auth/
│   │   └── models.rs       # Credentials
│   ├── databae.rs          # DB placeholder helpers
│   └── bin/                # One main() per topic
│       ├── async.rs
│       ├── channel.rs
│       └── ...
```

## Learning path (suggested order)

1. **Basics** — `print`, `loop`, `match`, `enum`, `struct`
2. **Modules** — `mods`, then the **default crate**: open `lib.rs`, `main.rs`, and `auth` / `databae` to see `pub`, paths, and file-backed modules
3. **Ownership** — `borrow`, `lifetime`, `trait`, `question_operator`
4. **Concurrency** — `thread`, `channel`, `async`

## Contributing

Personal learning repo. Fork and adapt; swap badges or add experiments as you like.

