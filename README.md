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

and then as the subjext suggested I refactored to a ast, a binary tree where each node has two children like a & b. but the 'and' property is associative, so this could be a regular tree as well (not implemented yet). We can add a debug description for the tree which can print the tree also not yet implemented but possible.
However, it is a bit nonsense to use a regular tree in this case, since I use the polish notation and this means I expect two operands like 110|& would be (1 | 0) & 1. using a regular tree I would not know if the or takes two or tree operands... like (1 | 0 | 1) & ??


## Ex05 
Since you have already built the **AST (Tree)**, you are in a perfect position. Converting to **Negation Normal Form (NNF)** is essentially a "Tree-to-Tree" transformation.

In NNF, negations (`!`) are only allowed to touch variables. To get there, you apply **De Morgan's Laws** and the **Double Negation Law** to "push" the NOT operators down from the top of the tree to the leaves.

---

### 1. The Transformation Rules

You need to handle three main scenarios for a `NOT` node:

| Case | Logical Rule | Transformation |
| --- | --- | --- |
| **Double Negation** |  | `Not(Not(A))`  `A` |
| **De Morgan (AND)** |  | `Not(And(A, B))`  `Or(Not(A), Not(B))` |
| **De Morgan (OR)** |  | `Not(Or(A, B))`  `And(Not(A), Not(B))` |

**Wait! What about `>` and `=`?**
Before applying NNF, you must eliminate Implication and Equivalence:

*  becomes 
*  becomes  (or similar).

---

### 2. How to implement it in Rust

You should write a recursive function `to_nnf(node: Node) -> Node`. The key is to handle the `Node::Not` case by looking at its **child**.

```rust
fn negate(node: Node) -> Node {
    match node {
        Node::Value(b) => Node::Value(!b),
        Node::Variable(c) => Node::Not(Box::new(Node::Variable(c))),
        Node::Not(child) => *child, // Double Negation: !!A -> A
        Node::And(l, r) => Node::Or(Box::new(negate(*l)), Box::new(negate(*r))), // !(A & B) -> !A | !B
        Node::Or(l, r) => Node::And(Box::new(negate(*l)), Box::new(negate(*r))), // !(A | B) -> !A & !B
        // For Ex05, you'll need to handle Imply/Equiv here too or convert them first!
        _ => todo!("Handle other operators"),
    }
}

pub fn to_nnf(node: Node) -> Node {
    match node {
        Node::Not(child) => negate(to_nnf(*child)),
        Node::And(l, r) => Node::And(Box::new(to_nnf(*l)), Box::new(to_nnf(*r))),
        Node::Or(l, r) => Node::Or(Box::new(to_nnf(*l)), Box::new(to_nnf(*r))),
        // Leaf nodes stay as they are
        other => other,
    }
}

```

---

### 3. The 42 Requirement: RPN Output

The subject asks for a **string** in RPN as the return value. You'll need a helper function to turn your tree back into a string:

```rust
fn tree_to_rpn(node: &Node) -> String {
    match node {
        Node::Value(b) => if *b { "1".to_string() } else { "0".to_string() },
        Node::Variable(c) => c.to_string(),
        Node::Not(child) => format!("{}!", tree_to_rpn(child)),
        Node::And(l, r) => format!("{}{}&", tree_to_rpn(l), tree_to_rpn(r)),
        // ... etc
    }
}

```

### Why your "Tree" approach is the winner:

Doing this with strings (regex or find/replace) is almost impossible because of nested parentheses logic. With the Tree, you are just moving "Boxes" around.

**Watch out for the "Order of Operations":**

1. **Convert** `>` and `=` into `&`, `|`, and `!`.
2. **Push** `!` down using the `negate` logic above.
3. **Simplify** double negations.

**Would you like me to help you with the specific logic for converting `Imply` () and `Equiv` () into NNF-ready structures?**

