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

## linting 
I will use the rust analyser extension for vscode. Also adding a `.vscode/settings.json` file with the recommended settings for the formatter and linter.

Great question! They're **different tools**:

**Formatters** - Fix code style/spacing:
- `rustfmt` - Reformats code to follow Rust style conventions
- Example: `fn foo(a:u32)->u32{return a;}` → `fn foo(a: u32) -> u32 { return a; }`

**Linters** - Find bugs/style issues:
- `clippy` - Suggests improvements and catches common mistakes
- Example: Warns about inefficient code, unused variables, better idioms, etc.

**In your settings:**
```json
"editor.formatOnSave": true,
"editor.defaultFormatter": "rust-lang.rust-analyzer"
```
This uses **rustfmt** (formatter) to auto-format on save.

```json
"rust-analyzer.checkOnSave.command": "clippy",
"editor.codeActionsOnSave": {
  "source.fixAll.clippy": "explicit"
}
```
This uses **clippy** (linter) to check and auto-fix warnings on save.

**Summary:**
- ✅ **Formatter** (`rustfmt`) = Makes code look pretty
- ✅ **Linter** (`clippy`) = Finds bugs & suggests improvements

Both run on save in your setup! 🦀

## boolean algebra

In Boolean algebra, **AND has higher precedence than OR**.

So `A & B | C` is evaluated as `(A & B) | C`, not `A & (B | C)`.

**Precedence order (highest to lowest):**
1. `!` (NOT) - highest
2. `&` (AND)
3. `|` (OR) - lowest
4. `^` (XOR)
5. `>` (IMPLY)
6. `=` (EQUIV) - lowest

**Examples:**
- `A & B | C` = `(A & B) | C`
- `A | B & C` = `A | (B & C)`
- `!A & B` = `(!A) & B`
- `A & B & C | D` = `((A & B) & C) | D`

This matches most programming languages and standard Boolean algebra notation! 

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
Gray code is used to prevent errors in hardware because only one bit changes at a time (e.g., going from 1 to 2 in binary is 01 to 10—two bits changed! In Gray code, it's 01 to 11).

The formula is incredibly simple using bitwise operators:
```
G=n⊕(n≫1)
```

## Ex03

I had to understand the => implication truth table:

This is one of those concepts that feels counterintuitive until you look at the **Truth Table**. In logic, this is known as **Material Implication**.

The best way to understand why is to think about what it means for a promise (an implication) to be **broken**.

---

### 1. The "Contract" Analogy

Imagine I make you a promise: **"If it rains (), then I will bring an umbrella ()."**

There are four possible scenarios:

1. **It rains (), and I bring an umbrella ():** I kept my promise. (**True**)
2. **It rains (), but I don't bring an umbrella ():** I broke my promise. (**False**)
3. **It doesn't rain (), but I bring an umbrella anyway ():** I didn't break my promise. (I'm just prepared). (**True**)
4. **It doesn't rain (), and I don't bring an umbrella ():** I didn't break my promise. (**True**)

Notice that the **only** time the statement is **False** is when the "If" () happens, but the "Then" () does not.

---

### 2. Comparing the Truth Tables

Let's look at the output of vs :

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| 0 | 0 | **1** | 1 | **1** () |
| 0 | 1 | **1** | 1 | **1** () |
| 1 | 0 | **0** | 0 | **0** () |
| 1 | 1 | **1** | 0 | **1** () |

The columns match perfectly.

---

### 3. The Logical Intuition

The expression basically says:

> "Either the condition () didn't happen, OR the result () did."

If is false, the whole thing is true (we don't care about ). This is called **vacuous truth**. If is true, then for the whole expression to be true, **must** be true. This is exactly what "If , then " means.

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

and then as the subject suggested I refactored to a ast, a binary tree where each node has two children like a & b. but the 'and' property is associative, so this could be a regular tree as well (not implemented yet). We can add a debug description for the tree which can print the tree also not yet implemented but possible.
However, it is a bit nonsense to use a regular tree in this case, since I use the polish notation and this means I expect two operands like 110|& would be (1 | 0) & 1. using a regular tree I would not know if the or takes two or tree operands... like (1 | 0 | 1) & ??

## Ex05 

Since you have already built the **AST (Tree)**, you are in a perfect position. Converting to **Negation Normal Form (NNF)** is essentially a "Tree-to-Tree" transformation.

In NNF, negations (`!`) are only allowed to touch variables. To get there, you apply **De Morgan's Laws** and the **Double Negation Law** to "push" the NOT operators down from the top of the tree to the leaves.

---

### 1. The Transformation Rules

You need to handle three main scenarios for a `NOT` node:

| Case | Logical Rule | Transformation |
| --- | --- | --- |
| **Double Negation** |  | `Not(Not(A))` → `A` |
| **De Morgan (AND)** |  | `Not(And(A, B))` → `Or(Not(A), Not(B))` |
| **De Morgan (OR)** |  | `Not(Or(A, B))` → `And(Not(A), Not(B))` |

**Wait! What about `>` and `=`?**
Before applying NNF, you must eliminate Implication and Equivalence:

* A > B becomes ¬A | B
* A = B becomes (A & B) | (¬A & ¬B)

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

## Ex06 - CNF and DNF

Here's a comprehensive explanation of CNF and DNF:

### CNF (Conjunctive Normal Form):

- A conjunction (AND) of disjunctions (OR)
- Format: (A | B | C) & (D | E) & (F)
- Example: AB|C& means (A | B) & C

### DNF (Disjunctive Normal Form):

- A disjunction (OR) of conjunctions (AND)
- Format: (A & B & C) | (D & E) | (F)
- Example: AB&CD&| means (A & B) | (C & D)

To convert NNF to CNF, use distributivity: Push OR down over AND

To convert NNF to DNF, use distributivity: Push AND down over OR

### Why CNF?

However, the "magic" of CNF is how we write the internals of each individual rule so the computer can understand them. While the rules are connected by AND, each rule itself must be expressed as a "Sum" (an OR).

Here is how those specific examples translate from "Human Rules" into "CNF Clauses":

1. "Every flight must have at least one Captain"

Imagine a flight has three possible crew members: Smith, Jones, and Brown.

Logical requirement: (Smith is Captain) OR (Jones is Captain) OR (Brown is Captain).

CNF Clause: (S∨J∨B)

Why it works: If the solver tries to set all three to "False," the clause becomes false, and the solver knows that's an invalid schedule.

2. "If it lands in Berlin, the crew must rest"

This is an Implication: Berlin⟹Rest.

As you learned in Ex05, an implication A⟹B is equivalent to ¬A∨B.

CNF Clause: (¬Berlin∨Rest)

Why it works: This says: "Either we didn't land in Berlin, OR we are resting." The only thing forbidden is landing in Berlin and not resting.

3. "Pilot A cannot fly more than 8 hours"

This is usually a "Mutual Exclusion" rule. If we have two shifts (S1,S2) that would total more than 8 hours, the rule is: "You cannot do both."

Logic: ¬(S1∧S2)

Applying De Morgan (Ex05 again!): ¬S1∨¬S2

CNF Clause: (¬S1∨¬S2)

Why it works: It forces the solver to pick S1, or S2, or neither—but never both.

### The Big Picture: The "Product of Sums"

When you combine them, the SAT solver sees one giant formula where every single "OR" clause must be satisfied simultaneously:

(Pilot₁ ∨ Pilot₂) ∧ (¬Berlin ∨ Rest) ∧ (¬S1 ∨ ¬S2) …

This is why your Ex06 is so important. A scheduler doesn't just need one rule; it needs to find a solution that satisfies all rules at once. By converting your logic into a "Product of Sums" (CNF), you are creating a checklist where the computer can't move on until every single "OR" bracket has at least one "True" inside it.

### CNF Distributivity

Of the two distributivity laws, only the second one is needed for CNF:

```
(A ∨ (B ∧ C)) ⇔ ((A ∨ B) ∧ (A ∨ C))
(A ∧ (B ∨ C)) ⇔ ((A ∧ B) ∨ (A ∧ C))
```

CNF requires the second rule, while DNF requires the first one.

Think of the name to remember which is which:

- **Conjunctive Normal Form (CNF)**: The "Main" connector is the Conjunction (AND). You want the ∧ on the outside.
- **Disjunctive Normal Form (DNF)**: The "Main" connector is the Disjunction (OR). You want the ∨ on the outside.

### The "CNF Algorithm" in 3 Steps:

1. **NNF First**: Run your negation_normal_form from Ex05.
2. **Simplify**: Ensure there are no >, =, or ^ left.
3. **Distribute OR over AND**:
   - Walk the tree recursively.
   - Every time you see Node::Or(left, right):
     - If right is an And(B, C), return And(Or(left, B), Or(left, C)).
     - If left is an And(A, B), return And(Or(A, right), Or(B, right)).

### Karnaugh Maps (K-Maps) - Optional Bonus

**Karnaugh Maps (K-maps)** are a visual method to **simplify Boolean expressions** by grouping terms to eliminate redundant variables.

#### What is CNF Simplification?

Your CNF might be **logically correct but redundant**. For example:

```
(A | B) & (A | !B) = A
```

A K-map helps you find these redundancies and produce a **minimal CNF**.

#### How K-maps Work

1. **Draw a truth table grid** (2D for 2-4 variables)
2. **Mark cells** where the formula is `true`
3. **Group adjacent 1s** in powers of 2 (1, 2, 4, 8...)
4. **Extract simplified terms** from each group

#### Example: `AB|A!B|`

```
    B  !B
A   1   1   <- Both cells are 1, so group them
!A  0   0
```

The group covers both `B` values → **A doesn't depend on B** → Simplified: `A`

#### Why It Matters

For digital circuit design:
- ✅ Fewer gates = cheaper hardware
- ✅ Faster circuits
- ✅ Lower power consumption

#### Implementation Notes

K-map simplification is **complex** because:
- You need to handle 4+ variables (3D/4D grids)
- Finding optimal groupings is NP-hard
- Tools like Quine-McCluskey algorithm automate this

**For your project:** Implementing K-map simplification is **optional** but impressive. If you do:
1. Convert CNF to truth table
2. Apply grouping algorithm
3. Generate minimal CNF from groups

#### A Brief History of K-Maps

A **Karnaugh Map (K-Map)** is a visual method used to simplify Boolean algebra expressions without having to struggle through complex algebraic theorems or the recursive "explosions" of the distributive law.

Invented by Maurice Karnaugh in 1953, it is essentially a **truth table rearranged into a 2D grid** where the cells are ordered using **Gray Code** (only one bit changes between adjacent cells).

#### Why is it noteworthy?

##### 1. Visual Pattern Recognition vs. Algebraic Grunt Work

In your current Rust project, you are using the **Distributive Law**. As you've seen, it explodes into multiple clauses. A K-Map allows you to look at the "1s" (or "0s") on a grid and circle groups of 2, 4, or 8 cells. Each circle represents a simplified term.

* **The "Magic":** If a variable changes state (e.g., goes from 0 to 1) within a circle, that variable is redundant and can be deleted.

##### 2. Minimization (Optimal CNF/DNF)

Your current recursive "Distributor" function might produce a correct CNF, but it won't necessarily be the **shortest** one. K-Maps are noteworthy because they guarantee the **minimal** form of a Boolean function, which is critical in hardware design to save on physical logic gates and reduce power consumption.

##### 3. Gray Code Adjacency

K-Maps use a specific ordering (00, 01, 11, 10) so that moving from one cell to the next only changes one variable. This "wraps around" like a torus (the top edge is adjacent to the bottom edge).

#### K-Map vs. Your 42 Project

In the context of **Ready Set Boole**, the evaluators aren't expecting you to implement a K-Map algorithm (which is quite hard to code). They want to see:

1. **NNF:** NOTs pushed to variables.
2. **CNF:** Distributing OR over AND.

**K-Maps are the "Human way"** to solve this on paper during an exam. **The Distributive Law is the "Compiler way"** to solve it in code.

## Ex07 - SAT (Boolean Satisfiability)

### What is SAT?

A Boolean formula is **satisfiable** if there exists at least one assignment of variable values that makes the entire formula evaluate to `true`.

### SAT Classification

- **Satisfiable**: At least one row in the truth table outputs `true`
- **Unsatisfiable (Contradiction)**: All rows output `false` (e.g., `A & !A`)
- **Tautology**: All rows output `true` (always satisfiable)

### Algorithm

1. Parse the RPN formula into an Abstract Syntax Tree (AST)
2. Extract all variables from the formula
3. Generate a complete truth table (all 2^n variable combinations)
4. Check if any row evaluates to `true`
5. Return `true` if found, `false` otherwise

### Time Complexity

O(2^n) where n is the number of variables (exponential)

## Ex08 - Powerset

### What is a Powerset?

A Powerset of a set S is the set of all possible subsets, including the empty set and S itself. If your set has n elements, the powerset will have 2^n elements.

### Example

For set [1, 2, 3], the powerset contains 8 subsets:
```
[]
[1]
[2]
[1, 2]
[3]
[1, 3]
[2, 3]
[1, 2, 3]
```

### The Binary Enumeration Algorithm

This is a classic computer science technique that uses **binary counting** to generate powersets efficiently:

```rust
pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    let n = set.len();
    let num_subset = 1 << n;  // 2^n
    let mut results = Vec::with_capacity(num_subset);

    for i in 0..num_subset {
        let mut subset: Vec<i32> = Vec::new();
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                subset.push(set[j]);
            }
        }
        results.push(subset);
    }
    results
}
```

### How It Works

The key insight: **each subset corresponds to a binary number**.

For a set of `n` elements, you have `2^n` subsets. Each subset can be represented as an `n`-bit binary number:

```
For [1, 2, 3]:
Binary  Decimal  Subset
000   →   0    → []
001   →   1    → [1]
010   →   2    → [2]
011   →   3    → [1, 2]
100   →   4    → [3]
101   →   5    → [1, 3]
110   →   6    → [2, 3]
111   →   7    → [1, 2, 3]
```

The algorithm iterates from 0 to 2^n - 1. For each number i:
- Check each bit position j
- If bit j is set, include element j in the subset

### Why It's Amazing

- ✅ **O(n·2^n) time** — optimal for powerset generation
- ✅ **Very efficient** — just bit operations (`>>`, `&`)
- ✅ **Natural ordering** — generates in binary order
- ✅ **No recursion** — iterative, so no stack overhead

### Historical Origin

This comes from **combinatorics** and **discrete mathematics**. The technique is sometimes called:
- **Binary enumeration**
- **Bitmask iteration**
- **Gray code variant** (if ordered differently)

It's taught in algorithms courses and used in competitive programming for subset problems! 🎯