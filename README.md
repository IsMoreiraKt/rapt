# rtapt
rtapt is a submodule of the [RTest](https://github.com/IsMoreiraKt/RTest) library.

It provides over 750 assertion functions for Rust's primitive and standard types, ensuring robust and reliable testing.

<br>

### Features
1. Extensive assertion support for a wide range of types.
2. Panic-based failure mechanism for strict test validation.
3. Lightweight and independent, can be used separately from RTest.
4. Detailed error messages for improved debugging.

<br>

### Supported Types
**rtapt** includes assertions for the following types:

| Category                | Types                                                                                  |
|-------------------------|----------------------------------------------------------------------------------------|
| **Booleans**            | `bool`                                                                                 |
| **Characters**          | `char`                                                                                 |
| **Integers**            | `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize` |
| **Floating Point**      | `f32`, `f64`                                                                           |
| **Complex Numbers**     | `num::Complex<T>`                                                                      |
| **Strings**             | `&str`, `String`, `Regex`                                                              |
| **Collections**         | `Vec<T>`, `[T; N]`, `HashSet<T>`, `BTreeSet<T>`, `HashMap<K, V>`, `BTreeMap<K, V>`     |
| **Ranges**              | `Range<T>`                                                                             |
| **Option & Result**     | `Option<T>`, `Result<T, E>`                                                            |
| **Smart Pointers**      | `Box<T>`, `Rc<T>`, `Arc<T>`                                                            |
| **Interior Mutability** | `Cell<T>`, `RefCell<T>`                                                                |

<br>

### Installation
To use **rtapt**, install it via `Cargo`:

```bash
cargo add rtapt
```

Or add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
rtapt = "0.0.1"
```

<br>

### Usage
Here’s a basic example of using rtapt assertions:

```rust
use rtapt::assert_str_eq;

fn main() {
    assert_str_eq!("Hello World", "Hello World"); // ok
    assert_str_eq!("Hello World", "hello world"); // panic
}
```

<br>

### Error Handling
By default, **rtapt** assertions trigger a **panic** when they fail.

This ensures that failed tests do not continue execution.

Future updates may introduce alternative failure handling mechanisms.

<br>

### Contribution
Thank you for considering contributing to rtapt! Before getting started, please read the [LICENSE](./LICENSE) file.

The main repository includes a dedicated `dev-docs` directory, which contains:

- Contribution guidelines.
- Security reporting instructions.
- Pull request standards.
- Guides on writing effective issues.

Make sure to check these resources before submitting any changes. Welcome to the community!

<br>

### License
This project is licensed under **LGPL-2.1-or-later**. See the [LICENSE](./LICENSE) file for more details.
