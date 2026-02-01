# Ready-Set-Boole
Ready, Set, Boole!


## workspaces

As described in the docs I create a directory for the workspace:
```
$ mkdir ready_set_boole
$ cd ready_set_boole
```

Then I manually create the Cargo.toml file:
```yaml
[workspace]
resolver = "3"
```

Because the subject asks me for a "main" to test the single exercises I will add a binary called ready_set_boole_main just to run and play with the whole thing, Not really needed because each module-exercise will have a main, a lib.rs file with functions and tests.

after creating the files manually I just need to add :
```rust
cargo new ex00 --lib
```
and so on for each new exercice. I will add the main.rs file manually as required by the subject so everyone is happy.

## comments

### Documentation with `///`

Rust uses `///` for documentation comments that generate HTML documentation. These go above the item you're documenting:

```rust
/// Adds two numbers using only bitwise operators.
///
/// # Arguments
///
/// * `a` - The first number to add
/// * `b` - The second number to add
///
/// # Examples
///
/// ```
/// use ex00::adder;
/// let answer = adder(2, 2);
/// assert_eq!(4, answer);
/// ```
pub fn adder(a: u32, b: u32) -> u32 {
    // implementation
}
```

### Testing Documentation Examples

Code blocks in doc comments are automatically tested by `cargo test`:

```bash
$ cargo test --doc
```

This will run all the code examples in your `///` comments to ensure they compile and work correctly.

### Generating and Viewing Documentation

To generate and open the HTML documentation in your browser:

```bash
$ cargo doc --open
```

Or if you're in a workspace, specify the package:

```bash
$ cargo doc --package ex00 --open
```

This will:
1. Generate HTML documentation in `target/doc/`
2. Automatically open it in your default browser
3. Include all public items with their documentation

## Ex00

it will have this function signature in ex00:
```rust
fn adder(a: u32, b: u32) -> u32;
```

and I will add the following in the "main" top binary module so it will find the functions:
```
[package]
name = "ready_set_boole_main"
version = "0.1.0"
edition = "2024"

[dependencies]
ex00 = { path = "../ex00" }
```

This is in the cargo.toml in the ready_set_boole_main module which is just to play with the code and test all those modules together.

