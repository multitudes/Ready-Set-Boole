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

## ex02 - Gray code
Gray code is used to prevent errors in hardware because only one bit changes at a time (e.g., going from 1 to 2 in binary is 01 to 10—two bits changed! In Gray code, it’s 01 to 11).

The formula is incredibly simple using bitwise operators:
```
G=n⊕(n≫1)
```

## Ex03

I had to understand the => implication truth table:

This is one of those concepts that feels counterintuitive until you look at the **Truth Table**. In logic, this is known as **Material Implication**.

The best way to understand why  is to think about what it means for a promise (an implication) to be **broken**.

---

### 1. The "Contract" Analogy

Imagine I make you a promise: **"If it rains (), then I will bring an umbrella ()."**

There are four possible scenarios:

1. **It rains (), and I bring an umbrella ():** I kept my promise. (**True**)
2. **It rains (), but I don't bring an umbrella ():** I broke my promise. (**False**)
3. **It doesn't rain (), but I bring an umbrella anyway ():** I didn't break my promise. (I’m just prepared). (**True**)
4. **It doesn't rain (), and I don't bring an umbrella ():** I didn't break my promise. (**True**)

Notice that the **only** time the statement is **False** is when the "If" () happens, but the "Then" () does not.

---

### 2. Comparing the Truth Tables

Let's look at the output of  versus :

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| 0 | 0 | **1** | 1 | **1** () |
| 0 | 1 | **1** | 1 | **1** () |
| 1 | 0 | **0** | 0 | **0** () |
| 1 | 1 | **1** | 0 | **1** () |

The columns match perfectly.

---

### 3. The Logical Intuition

The expression  basically says:

> "Either the condition () didn't happen, OR the result () did."

If  is false, the whole thing is true (we don't care about ). This is called **vacuous truth**. If  is true, then for the whole expression to be true,  **must** be true. This is exactly what "If , then " means.

### implementation - stack
At first I did a stack based approach. 
```rust
pub fn eval_formula(formula: &str) -> bool {
    let mut stack: Vec<bool> = Vec::new();

    for c in formula.chars() {
        match c {
            '0' => stack.push(false),
            '1' => stack.push(true),
            '&' | '|' | '^' | '>' | '=' => {
                let b = stack.pop().expect("Empty stack");
                let a = stack.pop().expect("Empty stack");
                stack.push(match c {
                    '&' => a & b,
                    '|' => a | b,
                    '^' => a ^ b,
                    '>' => !a | b, // Logical implication: A => B is same as !A | B
                    '=' => a == b, // Logical equivalence
                    _ => unreachable!(),
                    });
            }
            _ => continue, // Ignore whitespace or invalid chars if necessary
        }
    }
    // if the formula is correct then I just have one value in the stack left
    stack.pop().expect("Final stack is empty")
}
```

and then as the subjext suggested I refactored to a ast.