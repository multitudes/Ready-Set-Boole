# Ready-Set-Boole
Ready, Set, Boole!

The term "Boolean algebra" honors George Boole (1815–1864), a self-educated English mathematician. He introduced the algebraic system initially in a small pamphlet, The Mathematical Analysis of Logic, published in 1847 in response to an ongoing public controversy between Augustus De Morgan and William Hamilton, and later as a more substantial book, The Laws of Thought, published in 1854. Boole's formulation differs from that described above in some important respects. For example, conjunction and disjunction in Boole were not a dual pair of operations. Boolean algebra emerged in the 1860s, in papers written by William Jevons and Charles Sanders Peirce.

The first systematic presentation of Boolean algebra and distributive lattices is owed to the 1890 Vorlesungen of Ernst Schröder.

## Rust workspaces

To organize my project whch is a collectio of exercises I will create workspaces.
As described in the Rust docs I first create a directory for the workspace:

``` bash
mkdir ready_set_boole
cd ready_set_boole
```

Then I manually create the Cargo.toml file:

```yaml
[workspace]
resolver = "3"
```

after creating the files manually I just need to type in the terminal:

```rust
cargo new ex00 --lib
```

and so on for each new exercice. I will add the main.rs file manually as required by the subject.

## linting / formatter

I will use the rust analyser extension for vscode. Also adding a `.vscode/settings.json` file with the recommended settings for the formatter and linter. They are different tools.

**Formatters** - Fix code style/spacing:

- `rustfmt` - Reformats code to follow Rust style conventions
- Example: `fn foo(a:u32)->u32{return a;}` → `fn foo(a: u32) -> u32 { return a; }`

**Linters** - Find bugs/style issues:

- `clippy` - Suggests improvements and catches common mistakes
- Example: Warns about inefficient code, unused variables, better idioms, etc.

In my settings.json for vscode:

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

## boolean algebra precedences:

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

## Rust Documentation with `///`

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
cargo test --doc
```

This will run all the code examples in your `///` comments to ensure they compile and work correctly.

### Generating and Viewing Documentation

To generate and open the HTML documentation in your browser:

```bash
cargo doc --open
```

Or if you're in a workspace, specify the package:

```bash
cargo doc --package ex00 --open
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

```yaml
[package]
name = "ready_set_boole_main"
version = "0.1.0"
edition = "2024"

[dependencies]
ex00 = { path = "../ex00" }
```

This is in the cargo.toml in the ready_set_boole_main module which is just to play with the code and test all those modules together.

## ex00 - Addition

Implements addition using only bitwise operations (no `+` operator). Uses the binary addition algorithm with XOR for sum and AND for carry propagation.

**Algorithm:** :

```pseudocode
while b > 0:
carry = a AND b // Find bits that generate a carry
a = a XOR b // Sum without carry
b = carry << 1 // Shift carry left for next position
return a
```

1. XOR computes the sum without carry
2. AND (shifted left) computes the carry
3. Repeat until no carry remains

**Time Complexity:** O(log max(a,b)) ≈ O(32) = O(1) for u32

**Example:**

```text
5 + 3 = 8
0101 (5)
0011 (3)
Step 1: XOR = 0110 (sum without carry)
AND<<1 = 0010 (carry)
Step 2: XOR = 0100 (sum without carry)
AND<<1 = 0100 (carry)
Step 3: XOR = 0000 (sum without carry)
AND<<1 = 1000 (carry = 8)
Result: 1000 = 8
```

See `ex00/src/lib.rs` for detailed documentation.

## ex01 - Multiplication

Implements multiplication using only bitwise operations (no `*` operator). Uses the binary long multiplication algorithm: shift and add based on each bit of the multiplier.

**Algorithm:** For each bit set in `b`, add `a << bit_position` to the result.

```pseudocode
result = 0
while b > 0:
if b AND 1 == 1: // Check if lowest bit of b is set
result = result + a // Add current value of a to result
a = a << 1 // Shift a left (multiply by 2)
b = b >> 1 // Shift b right (divide by 2)
return result
```

**Time Complexity:** O(log b) = O(32) = O(1) constant time for u32

1. O(log b) — The General Case. 

The algorithm processes each bit of the multiplier b:
b has at most log₂(b) bits.  

Example: b = 1000 has ⌈log₂(1000)⌉ = 10 bits. 
The loop runs once per bit → O(log₂ b) iterations. 
2. O(32) — For u32 Specifically.

For u32 type:

Maximum value: 2³² - 1.
Number of bits: exactly 32 bits.
So log₂(u32::MAX) = 32.
The loop runs at most 32 times → O(32).

**Example:**

``` math
3 × 5 = 15

0011 (3)
× 0101 (5)
0011 ← 3 × bit₀
0000 ← 3 × bit₁ (shifted)
0011 ← 3 × bit₂ (shifted)
1111 = 15
```

See `ex01/src/lib.rs` for detailed documentation.

## ex02 - Gray code

Gray code is used to prevent errors in hardware because only one bit changes at a time (e.g., going from 1 to 2 in binary is 01 to 10—two bits changed! In Gray code, it's 01 to 11).

The prototype of the function to write is the following: `fn gray_code(n: u32) -> u32;`  

The formula is incredibly simple using bitwise operators:

```bool
G=n⊕(n≫1)
```

## Ex03

> write a function that takes as input a string that contains a propositional formula in reverse polish notation, evaluates this formula, then returns the result.

## Operator Symbols

| Symbol | Mathematical Equivalent | Description |
|--------|------------------------|-------------|
| `0` | ⊥ | false |
| `1` | ⊤ | true |
| `!` | ¬ | Negation |
| `&` | ∧ | Conjunction |
| `\|` | ∨ | Disjunction |
| `^` | ⊕ | Exclusive disjunction |
| `>` | ⇒ | Material condition |
| `=` | ⇔ | Logical equivalence |

Two of those operators were new to me:

### Material condition 

The => implication in logic, is known as **Material Implication** as well. Imagine I make you a promise: **"If it rains (), then I will bring an umbrella ()."**

There are four possible scenarios:

1. **It rains (), and I bring an umbrella ():** I kept my promise. (**True**)
2. **It rains (), but I don't bring an umbrella ():** I broke my promise. (**False**)
3. **It doesn't rain (), but I bring an umbrella anyway ():** I didn't break my promise. (I'm just prepared). (**True**)
4. **It doesn't rain (), and I don't bring an umbrella ():** I didn't break my promise. (**True**)

Notice that the **only** time the statement is **False** is when the "If" () happens, but the "Then" () does not.

Let's look at the output of vs :

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| 0 | 0 | **1** | 1 | **1** () |
| 0 | 1 | **1** | 1 | **1** () |
| 1 | 0 | **0** | 0 | **0** () |
| 1 | 1 | **1** | 0 | **1** () |

The expression basically says:
> "Either the condition () didn't happen, OR the result () did."

If is false, the whole thing is true (we don't care about ). This is called **vacuous truth**. If is true, then for the whole expression to be true, **must** be true. This is exactly what "If , then " means.

### Logical Equivalence (⇔)

The `=` symbol represents **Logical Equivalence** (also written as ⇔ or ≡). It means "if and only if" (iff).

**Definition:** Two statements are equivalent when they always have the **same truth value**.

**Example:** "The light is on ⇔ The switch is up"

This means:
- If the light is on, then the switch must be up
- If the switch is up, then the light must be on
- They **always agree**

#### Truth Table

| A | B | A = B | Meaning |
|---|---|-------|---------|
| 0 | 0 | **1** | Both false → They agree ✓ |
| 0 | 1 | **0** | Different → They disagree ✗ |
| 1 | 0 | **0** | Different → They disagree ✗ |
| 1 | 1 | **1** | Both true → They agree ✓ |

**Key insight:** `A = B` is true when A and B have the **same value** (both true OR both false).

#### Equivalence vs Equality

**In logic:**
```
A = B  means  "A and B always match"
```

**Equivalent to:**

```
(A → B) ∧ (B → A)   "A implies B AND B implies A"
```

**Also equivalent to:**

```
(A ∧ B) ∨ (¬A ∧ ¬B)   "Both true OR both false"
```

#### Examples

**Example 1: Mathematical equivalence**

```
(x > 5) = (x ≥ 6)   for integers

True when x = 7:  (True = True)  → True ✓
True when x = 3:  (False = False) → True ✓
False when x = 5.5: Would break if x could be real!
```

**Example 2: Logical equivalence**

```
"It's raining" = "The ground is wet"   (in a controlled scenario)

Both true:  Raining AND ground wet → Equivalent ✓
Both false: Not raining AND ground dry → Equivalent ✓
One true, one false: NOT equivalent ✗
```

**Example 3: Circuit logic**
```
Switch A = Switch B   (for a two-way light switch)

Both ON:  Light is on → Equivalent ✓
Both OFF: Light is off → Equivalent ✓
One ON, one OFF: Light state depends on wiring ✗
```

#### Why It's Called "If and Only If"

**Breaking down "A ⇔ B":**

1. **"If A then B"** (A → B)
   - When A is true, B must be true
   
2. **"If B then A"** (B → A)  
   - When B is true, A must be true

3. **Together:** They **force each other** to have the same value

**Example:**

```text
"You pass the exam ⇔ You scored ≥ 60%"

If you pass → You scored ≥ 60% (first direction)
If you scored ≥ 60% → You pass (second direction)
They're tied together!
```

#### Connection to XOR (Exclusive OR)

**Important relationship:**

```text
A = B  is the OPPOSITE of  A ⊕ B

Equivalence = NOT(XOR)
¬(A ⊕ B) = (A = B)
```

**Comparison:**

| A | B | A ⊕ B (XOR) | A = B (Equivalence) |
|---|---|-------------|---------------------|
| 0 | 0 | 0 (same)    | **1** (agree) |
| 0 | 1 | 1 (different) | **0** (disagree) |
| 1 | 0 | 1 (different) | **0** (disagree) |
| 1 | 1 | 0 (same)    | **1** (agree) |

**XOR asks:** "Are they different?"  
**Equivalence asks:** "Are they the same?"

#### In Set Theory

In your Ex09 set evaluation, `A = B` means:

```text
Elements that belong to BOTH sets OR belong to NEITHER set

(A ∩ B) ∪ (A^c ∩ B^c)
```

**Example:**
```text
Universe: {1, 2, 3, 4}
A = {1, 2}
B = {2, 3}

A = B:
- Elements in both: {2} ✓
- Elements in neither: {4} ✓  
- Result: {2, 4}
```

**Why {2, 4}?**

- 1: In A but not B → Disagree ✗
- 2: In both → Agree ✓
- 3: In B but not A → Disagree ✗
- 4: In neither → Agree ✓

#### Practical Applications

**1. Digital Circuits (XNOR gate)**

```text
A = B creates an XNOR gate
Output is HIGH when inputs match
Used in equality checkers
```

**2. Password Verification**

```text
Stored Password = Entered Password
True only if they're exactly the same
```

**3. Synchronization**

```text
State A = State B
Used to check if two systems are in sync
```

**4. Mathematical Proofs**

```text
To prove A ⇔ B, you must prove:
1. A → B (forward direction)
2. B → A (backward direction)
```

#### Summary Table

| Aspect | Material Implication (→) | Logical Equivalence (⇔) |
|--------|-------------------------|------------------------|
| **Symbol** | `>` | `=` |
| **Meaning** | "If...then" | "If and only if" |
| **False when** | A true, B false | A and B differ |
| **True when** | B true OR A false | A and B match |
| **Composition** | ¬A ∨ B | (A → B) ∧ (B → A) |
| **Relationship** | One-way | Two-way (bidirectional) |

#### Visual Mnemonic

**Implication (→):**
```
A → B
"A forces B"
If A happens, B must happen
But B can happen without A
```

**Equivalence (⇔):**
```
A ⇔ B
"A and B are locked together"
If either changes, both must change
They always match
```

#### Mathematical Notation Variants

Different fields use different symbols:

| Symbol | Meaning | Usage |
|--------|---------|-------|
| `=` | Equivalence | Your exercise, some logic texts |
| `⇔` | Equivalence | Most logic textbooks |
| `≡` | Equivalence | Some mathematical logic |
| `↔` | Equivalence | Alternative arrow notation |
| `iff` | Equivalence | "If and only if" (written) |


### implementation

As the subject suggested I create an ast, an Abstract Syntax Tree like:

```text
      OR (|)
     /    \
   AND(&)  C
   /   \
  A     B
```

This is the first major exercise in the module. Here I start with my node implementation for the ast:
```rust
pub enum Node {
    /// A boolean constant: true (1) or false (0)
    Value(bool),
    ///Will be used in later exercises: holds variable 'A', 'B', etc.
    Variable(char),
    /// Logical NOT: ¬a
    Not(Box<Node>),
    /// Logical AND: a ∧ b
    And(Box<Node>, Box<Node>),
    /// Logical OR: a ∨ b
    Or(Box<Node>, Box<Node>),
    /// Logical XOR: a ⊕ b
    Xor(Box<Node>, Box<Node>),
    /// Material implication: a ⇒ b
    Imply(Box<Node>, Box<Node>),
    /// Logical equivalence: a ⇔ b
    Equiv(Box<Node>, Box<Node>),
}
```

## Ex04 - Truth Table

Generates and displays a complete truth table for a Boolean formula with variables.

**Input:** RPN formula with variables A-Z (e.g., `AB&C|` = `(A ∧ B) ∨ C`)

**Output:** Markdown truth table with all 2^n rows (n = number of variables)

**Algorithm:**
1. Parse RPN formula to AST
2. Extract all variables from the formula
3. Generate all 2^n variable combinations (truth assignments)
4. Evaluate formula for each combination
5. Display as formatted table

**Example:**
```
Formula: AB&C|
Variables: A, B, C (3 variables → 8 rows)

| A | B | C | Result |
|---|---|---|--------|
| 0 | 0 | 0 |   0    |
| 0 | 0 | 1 |   1    |
| 0 | 1 | 0 |   0    |
| 0 | 1 | 1 |   1    |
| 1 | 0 | 0 |   0    |
| 1 | 0 | 1 |   1    |
| 1 | 1 | 0 |   1    |
| 1 | 1 | 1 |   1    |
```

**Time Complexity:** O(2^n) where n = number of variables

**Space Complexity:** O(2^n) for storing the table

See `ex04/src/lib.rs` for detailed documentation.

## Ex05

Since I have already built the **AST (Tree)**, converting to **Negation Normal Form (NNF)** is essentially a "Tree-to-Tree" transformation.

In NNF, negations (`!`) are only allowed to touch variables. To get there, you apply **De Morgan's Laws** and the **Double Negation Law** to "push" the NOT operators down from the top of the tree to the leaves.

### 1. The Transformation Rules

We need to handle three main scenarios for a `NOT` node:

| Case | Transformation |
| --- | --- |
| **Double Negation** | `Not(Not(A))` → `A` |
| **De Morgan (AND)** | `Not(And(A, B))` → `Or(Not(A), Not(B))` |
| **De Morgan (OR)** | `Not(Or(A, B))` → `And(Not(A), Not(B))` |

What about `>` and `=`? 
Before applying NNF, we must eliminate Implication and Equivalence:

* A > B becomes ¬A | B
* A = B becomes (A & B) | (¬A & ¬B)

### 2. How to implement it in Rust

We will write a recursive function `ast_to_nnf(node: Node) -> Node`. The key is to handle the `Node::Not` case by looking at its **child**.  
The subject asks for a **string** in RPN as the return value. We'll need a helper function to turn your tree back into a string:

**The "Order of Operations":**

1. **Convert** `>` and `=` into `&`, `|`, and `!`.
2. **Push** `!` down using the `negate` logic above.
3. **Simplify** double negations.

## Ex06 - Conjunctive Normal Form (CNF)

Converts a Boolean formula to **Conjunctive Normal Form** - a standardized format required for SAT solvers and automated reasoning systems.

**Requirement:** Transform RPN formula so that:
- Every negation (`!`) appears directly after a variable
- Every conjunction (`&`) appears at the end of the formula
- Result is an AND of ORs: `(A|B|C) & (D|E) & (F)`

**Input:** RPN formula (e.g., `AB&!`)  
**Output:** Equivalent CNF formula (e.g., `A!B!|`)

### What is CNF?

**Conjunctive Normal Form (CNF)** is a standardized way to write Boolean formulas as:

```
(Clause₁) AND (Clause₂) AND (Clause₃) AND ...
```

Where each clause is an OR of literals:
```
(A | B | ¬C) ∧ (¬A | D) ∧ (B | ¬D)
```

**Key characteristics:**
- ✅ Outer operator: AND (conjunction) - the "Product"
- ✅ Inner operators: OR (disjunction) - the "Sums"
- ✅ Also called "Product of Sums"

**RPN Example:**
```
Formula: AB|C&
Infix: (A | B) & C
CNF: Already in CNF (AND at top level)
```

### CNF vs DNF

| Property | CNF (Conjunctive) | DNF (Disjunctive) |
|----------|------------------|-------------------|
| **Outer operator** | AND (∧) | OR (∨) |
| **Inner operators** | OR (\|) | AND (&) |
| **Structure** | (A\|B) & (C\|D) | (A&B) \| (C&D) |
| **Name** | Product of Sums | Sum of Products |
| **Used for** | SAT solvers | Circuit minimization |

**Memory trick:** 
- **C**NF = **C**onjunction (AND) on the outside
- **D**NF = **D**isjunction (OR) on the outside

### Why CNF Matters

CNF is the **standard input format** for SAT solvers because:

**1. Efficient evaluation:** Check if ANY clause fails (short-circuit)

**2. Natural constraint representation:**
```
Rule: "Every flight must have a pilot"
CNF: (PilotA | PilotB | PilotC)
Meaning: At least one must be true
```

**3. Real-world applications:**
- **Scheduling**: Aircraft crew assignments
- **Hardware verification**: Circuit correctness proofs
- **AI planning**: Action preconditions
- **Sudoku solvers**: Cell value constraints

### Real-World Examples

#### Example 1: Flight Scheduling

**Rule:** "Every flight must have at least one captain"

**Variables:** Smith (S), Jones (J), Brown (B)

**CNF clause:** `(S | J | B)`

**Why it works:** If all three are false, the clause fails → invalid schedule

#### Example 2: Constraint Implications

**Rule:** "If landing in Berlin, crew must rest"

**Implication:** `Berlin → Rest`

**Convert to CNF:** 
```
A → B  ≡  ¬A | B    (from ex05)
¬Berlin | Rest
```

**CNF clause:** `(¬Berlin | Rest)`

**Why it works:** Only fails when `Berlin=true` and `Rest=false`

#### Example 3: Mutual Exclusion

**Rule:** "Pilot cannot work both morning and evening shifts"

**Logic:** `¬(Morning ∧ Evening)`

**Apply De Morgan's Law:**
```
¬(A ∧ B) ≡ ¬A | ¬B
```

**CNF clause:** `(¬Morning | ¬Evening)`

**Why it works:** Forces at most one shift to be true

### The Algorithm

**Step 1: Convert to NNF** (use ex05)
- Push all negations down to variables
- Eliminate `>` and `=` operators

**Step 2: Apply Distributivity**
- Use the law: `A | (B & C) ≡ (A | B) & (A | C)`
- Recursively distribute OR over AND

**Step 3: Flatten to RPN**
- Convert AST back to RPN string
- Ensure ANDs appear at the end

### Distributivity Law for CNF

The **key transformation** is:

```
A ∨ (B ∧ C) ⟺ (A ∨ B) ∧ (A ∨ C)
```

**Visual example:**
```
      OR                AND
     /  \              /   \
    A   AND    →    OR     OR
       /  \         / \    / \
      B   C        A  B   A  C
```

**Why this works:**
```
Truth: A is true, OR (both B and C are true)
Equivalent: (A is true OR B is true) AND (A is true OR C is true)
```

**Opposite law (for DNF):**
```
A ∧ (B ∨ C) ⟺ (A ∧ B) ∨ (A ∧ C)
```

### Implementation Sketch

```rust
pub fn conjunctive_normal_form(formula: &str) -> String {
    // Parse RPN to AST
    let tree: Node = match parse_rpn(formula) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error parsing formula: {}", e);
            std::process::exit(1);
        }
    };
    // Transform to NNF
    let nnf_tree = ast_to_nnf(&tree);
    // Transform to CNF
    let cnf_tree = to_cnf(nnf_tree);
    // Convert back to RPN
    ast_to_rpn(&cnf_tree)
}
```

### Example Transformation

**Input:** `AB&!`  
**Infix:** `¬(A ∧ B)`

**Step 1 - NNF (De Morgan's Law):**
```
¬(A ∧ B) → ¬A ∨ ¬B
```

**Step 2 - Already in CNF:**
```
(¬A ∨ ¬B)  ← Single clause
```

**Output:** `A!B!|`

**Complex example:**

**Input:** `AB|CD|&`  
**Infix:** `(A ∨ B) ∧ (C ∨ D)`

**Already in CNF:**
```
Two clauses ANDed together
(A ∨ B) ∧ (C ∨ D)
```

**Output:** `AB|CD|&` (unchanged)

**Distribution example:**

**Input:** `AB&C|`  
**Infix:** `(A ∧ B) ∨ C`

**Apply distributivity:**
```
(A ∧ B) ∨ C  →  (A ∨ C) ∧ (B ∨ C)
```

**Output:** `AC|BC|&`

### Time Complexity

**Worst case:** Exponential in formula size

**Why?** Distributivity can cause exponential growth:

```
(A₁ ∨ B₁) ∧ (A₂ ∨ B₂) ∧ ... ∧ (Aₙ ∨ Bₙ)

Distributed fully → 2ⁿ clauses
```

**Practical tip:** Most real-world formulas don't hit worst case

### Karnaugh Maps (K-Maps) - Optional Bonus

**K-Maps** are a visual method to **simplify** CNF/DNF by eliminating redundant terms.

#### What is K-Map Simplification?

Your CNF might be correct but **redundant**:

```
(A | B) & (A | ¬B) = A   ← Can be simplified
```

K-Maps find these redundancies to produce **minimal CNF**.

#### How K-Maps Work

1. Draw truth table as 2D grid (Gray code ordering)
2. Mark cells where formula = 1
3. Group adjacent 1s in powers of 2 (1, 2, 4, 8...)
4. Extract simplified terms from groups

**Example: `AB|A!B|` → `A`**

```
    B  ¬B
A   1   1   ← Both cells are 1, group them
¬A  0   0
```

Group covers both B values → **B is irrelevant** → Result: `A`

#### Why K-Maps Matter

For **digital circuit design:**
- ✅ Fewer logic gates = cheaper hardware
- ✅ Faster circuits (fewer propagation delays)
- ✅ Lower power consumption

#### K-Map vs Your Implementation

**For Ready Set Boole:**
- Evaluators expect: **Correct CNF** (not necessarily minimal)
- K-Map simplification: **Optional bonus** (complex to implement)

**Comparison:**

| Approach | Use Case | Difficulty |
|----------|----------|------------|
| **Distributive Law** | Generate CNF (your code) | Medium |
| **K-Maps** | Simplify by hand (2-4 vars) | Easy |
| **Quine-McCluskey** | Automate simplification | Hard |

**History:** Invented by Maurice Karnaugh (1953), K-Maps use Gray code ordering so adjacent cells differ by only one variable.

See `ex06/src/lib.rs` for detailed implementation.

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

```text
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

### Historical Origin

This comes from **combinatorics** and **discrete mathematics**. The technique is sometimes called:

- **Binary enumeration**
- **Bitmask iteration**
- **Gray code variant** (if ordered differently)

## Boolean Lattices

### What is a Lattice?

A **lattice** is a partially ordered set (poset) in which every pair of elements has:

1. A **least upper bound** (supremum, or "join") - denoted ∨
2. A **greatest lower bound** (infimum, or "meet") - denoted ∧

Think of it as a structure where you can always find:

- The "smallest thing that's bigger than both" (join)
- The "biggest thing that's smaller than both" (meet)

### Visual Example: The Powerset Lattice

The powerset you generated in Ex08 forms a **Boolean lattice**! Here's the lattice for {1, 2}:

```text
        {1, 2}         ← Top (universal set)
         /  \
      {1}   {2}        ← Single elements
         \  /
          {}           ← Bottom (empty set)
```

**Ordering**: A ⊆ B means "A is below B in the lattice"

**Operations**:

- Join (∨): {1} ∨ {2} = {1, 2} (union)
- Meet (∧): {1} ∧ {2} = {} (intersection)

### The Full {1, 2, 3} Boolean Lattice

```text
                {1,2,3}
              /   |   \
          {1,2} {1,3} {2,3}
           /|\   /|\   /|\
         {1} {2} {3}
           \  |  /
              {}
```

### Properties of Boolean Lattices

A **Boolean lattice** (or Boolean algebra) has these special properties:

1. **Complementation**: Every element has a complement
   - {1} ∪ {2,3} = {1,2,3}
   - {1} ∩ {2,3} = {}

2. **Distributivity**:
   - A ∨ (B ∧ C) = (A ∨ B) ∧ (A ∨ C)
   - A ∧ (B ∨ C) = (A ∧ B) ∨ (A ∧ C)

3. **De Morgan's Laws**:
   - ¬(A ∧ B) = ¬A ∨ ¬B
   - ¬(A ∨ B) = ¬A ∧ ¬B

4. **Identity Elements**:
   - Top (⊤): {1,2,3} (universal set)
   - Bottom (⊥): {} (empty set)

5. **Size**: A Boolean lattice with n atoms has exactly 2^n elements

### How to Tell if Something is a Lattice

**Test 1: Does every pair have a join and meet?**

Take any two elements. Can you find:

- Their least upper bound?
- Their greatest lower bound?

**Example - This IS a lattice:**

``` bool
    6
   / \
  2   3
   \ /
    1
```

- join(2,3) = 6 ✓
- meet(2,3) = 1 ✓

**Example - This is NOT a lattice:**

``` bool
    ?
   / \
  2   3
  |   |
  4   9
   \ /
    1
```

- What's join(2,3)? Could be 6, 12, 18... no unique least upper bound! 

Test 2: Check the Hasse diagram

Draw the partial order as a directed graph (Hasse diagram):

- Every "fork" must rejoin at exactly one element above
- Every "merge" must split from exactly one element below

### Boolean Lattices vs General Lattices

Not all lattices are Boolean:

**Boolean Lattice** (like powersets):

``` bool
- Has complements
- Is distributive
- Has 2^n elements for n atoms
- Examples: Powerset, Boolean circuits
```

**Non-Boolean Lattice** (like divisibility):

``` bool
Divisors of 12: {1, 2, 3, 4, 6, 12}

    12
   / \
  4   6
  | / |
  2   3
   \ /
    1
```

This is a lattice (join = LCM, meet = GCD) but NOT Boolean:

- **No complement for 2**: We need `x` where `LCM(2, x) = 12` AND `GCD(2, x) = 1`
  - Try 6: `LCM(2, 6) = 6` ✗ (not 12), `GCD(2, 6) = 2` ✗ (not 1)
  - Try 3: `LCM(2, 3) = 6` ✗ (not 12), `GCD(2, 3) = 1` ✓ (but LCM fails)
  - No element in the lattice satisfies both conditions!
- Not 2^n elements (has 6 elements, not a power of 2)

### Connection to the Project

Your Boolean algebra exercises are working inside a Boolean lattice:

- **Ex00-Ex03**: Operations (∧, ∨, ¬) in the 2-element lattice {0, 1}
- **Ex05 (NNF)**: Pushing ¬ down preserves lattice structure
- **Ex06 (CNF)**: Distributivity law from Boolean lattices
- **Ex07 (SAT)**: Finding if formula reaches ⊤ (true)
- **Ex08 (Powerset)**: Building the entire Boolean lattice!

**Mathematical Note**: Every finite Boolean lattice is isomorphic to the powerset lattice of some finite set. This deep connection is why your Ex08 powerset exercise is fundamental to understanding Boolean algebra!

## **Material Implication**

In logic, the **Material Implication**  is defined as "If A, then B." When we translate this into Set Theory, it represents the relationship: **"Everything that is NOT in A, OR everything that is in B."**

The formula for this is:  (where  is the Universe).

### Why the result is `[2, 3]` in your example:

Let’s break it down step-by-step using your specific inputs:

* **Set A:** `{1, 2}`
* **Set B:** `{2, 3}`
* **Universe ():** `{1, 2, 3}` (The union of all elements involved)

#### Step 1: Find "NOT A" (The complement)

"NOT A" () means all elements in the Universe that are **not** in Set A.

* Universe is `{1, 2, 3}`.
* A is `{1, 2}`.
* ** is `{3}**`.

#### Step 2: Perform the "OR" (Union) with B

Now we take the result of  and combine it with everything in Set B.

*  is `{3}`.
* B is `{2, 3}`.
* `{3} | {2, 3}` results in `{2, 3}`.

| A | B |  |
| --- | --- | --- |
| 1 | 1 | **1** (Element is in both) |
| 1 | 0 | **0** (Element in A but NOT in B — The only "False" case) |
| 0 | 1 | **1** (Element not in A, but in B) |
| 0 | 0 | **1** (Element in neither) |

**Let's check our numbers against the table:**

* **Number 1:** In A (1), Not in B (0). Table says **0**. (1 is excluded).
* **Number 2:** In A (1), In B (1). Table says **1**. (**2 is included**).
* **Number 3:** Not in A (0), In B (1). Table says **1**. (**3 is included**).

In this specific example `[[1, 2], [2, 3]]`, the result for **Equivalence** (`=`) and **Intersection** (`&`) is exactly the same: `[2]`.

But they are doing very different "math" behind the scenes. The difference only appears when there are elements in the **Universe** that **neither** set contains.

### The "Neither" Difference

Imagine this scenario:

* **Universe ():** `{1, 2, 3, 4}`
* **Set A:** `{1}`
* **Set B:** `{1}`

#### 1. Intersection (`AB&`)

* "What is in both?"
* **Result: `{1}**`

#### 2. Equivalence (`AB=`)

* "Where do they agree?"
* They agree on **1** (both have it).
* They **also** agree on **2, 3, and 4** (neither has them!).
* **Result: `{1, 2, 3, 4}**`

In this case, `AB=` gives you the whole Universe because  and  are identical. They agree on everything.

## bonus

### Reading Mathematical Function Notation

```
Let f be a function and let A be a set such as:
f : (x, y) ∈ [[0; 2¹⁶ - 1]]² ⊂ ℕ² → A
A ⊂ [0; 1] ⊂ ℝ
```

**Line 1: The Domain (Input)**

```text
f : (x, y) ∈ [[0; 2¹⁶ - 1]]² ⊂ ℕ²
```

Reading from right to left (as mathematicians build up):

1. **ℕ²** = "The Cartesian product ℕ × ℕ" = All pairs of natural numbers
   - "Natural numbers squared" or "2D grid of natural numbers"

2. **[[0; 2¹⁶ - 1]]²** = "The closed interval from 0 to 2¹⁶ - 1, squared"
   - This means: pairs (x, y) where both x and y are in [0, 65535]
   - In other words: {0, 1, 2, ..., 65535} × {0, 1, 2, ..., 65535}

3. **⊂** = "is a subset of"
   - Our specific range is a subset of all natural number pairs

4. **(x, y) ∈** = "the pair (x, y) belongs to"

5. **f :** = "the function f maps from"

**Full reading:**
> "f is a function that takes pairs (x, y) from the 2D grid of integers ranging from 0 to 2¹⁶ - 1 (which is 65535)"

---

**Line 2: The Codomain (Output Range)**

```
A ⊂ [0; 1] ⊂ ℝ
```

Reading from right to left:

1. **ℝ** = "The real numbers" = All numbers on the number line

2. **[0; 1]** = "The closed interval from 0 to 1"
   - All real numbers between 0 and 1, inclusive
   - Examples: 0, 0.5, 0.333..., 0.999..., 1

3. **A ⊂** = "A is a subset of"

**Full reading:**
> "The set A (where f maps to) is a subset of the interval [0, 1], which itself is a subset of all real numbers"

---

### Complete Translation

**In plain English:**

> "Let f be a function that maps pairs of integers (x, y), where both x and y range from 0 to 65535, into some set A. The set A contains real numbers between 0 and 1."

**In code terms (what you'd write in Rust):**

```rust
fn f(x: u16, y: u16) -> f64 {
    // x ranges from 0 to 65535
    // y ranges from 0 to 65535
    // Output is a float between 0.0 and 1.0
}
```

---

### Breaking Down the Notation

#### Domain Notation: [[0; 2¹⁶ - 1]]²

**[[a; b]]** = Closed interval of integers from a to b
- The double brackets [[...]] indicate **discrete** (integer) values
- Single brackets [...] would indicate **continuous** (real) values

**The "²" exponent:**
- Means "Cartesian product with itself"
- [[0; 2¹⁶ - 1]]² = [[0; 2¹⁶ - 1]] × [[0; 2¹⁶ - 1]]
- All possible pairs (x, y) where x and y are both in that range

**Why 2¹⁶ - 1?**
- 2¹⁶ = 65536 (the number of values a u16 can hold)
- 2¹⁶ - 1 = 65535 (the maximum value for u16)
- Range: [0, 65535] = exactly all u16 values

---

#### Codomain Notation: A ⊂ [0; 1] ⊂ ℝ

**[0; 1]** = Closed interval of reals from 0 to 1
- Single brackets [...] indicate **continuous** (real) values
- Includes 0, 1, and every real number in between
- Examples: 0, 0.5, π/4, √2/2, 1

**The chain of subsets:**
```
A ⊂ [0; 1] ⊂ ℝ

A is inside [0; 1], which is inside ℝ
```

This tells us:
1. A contains some (possibly all) numbers from [0, 1]
2. All numbers in A are real numbers
3. All numbers in A are between 0 and 1

---

### Connection to Your Ex10

In **Ex10 (Curve Saturation)**, this notation describes your space-filling curve function!

**The mathematical version:**
```
f : (x, y) ∈ [[0; 2¹⁶ - 1]]² ⊂ ℕ² → A
A ⊂ [0; 1] ⊂ ℝ
```

**Your Rust implementation:**
```rust
pub fn map(x: u16, y: u16, n: u16) -> f64 {
    // Domain: (x, y) where x, y ∈ [0, 65535]
    // Codomain: f64 value in [0.0, 1.0]
}
```

**What it means:**
1. **Input:** A coordinate pair (x, y) on a 65536 × 65536 grid
2. **Output:** A single real number between 0 and 1
3. **Purpose:** Map 2D discrete space to 1D continuous interval

This is the **inverse** of a typical space-filling curve!
- Typical: [0, 1] → [0, 1]² (1D to 2D)
- Your Ex10: [[0; 2¹⁶ - 1]]² → [0, 1] (2D to 1D)

---

### Visual Representation

```
Domain (Input Space):
┌─────────────────────┐
│  (0, 65535)  65535  │
│                     │  ← 2D Grid of integers
│                     │     65536 × 65536 points
│      (x, y)         │
│                     │
│  (0, 0)      65535  │
└─────────────────────┘

         ↓ f maps to

Codomain (Output Space):
├─────────────────────┤
0                     1  ← Real number line
                          Continuous values
                          
Example mappings:
f(0, 0) = 0.0
f(32767, 32767) ≈ 0.5
f(65535, 65535) = 1.0
```

---

### Notation Cheat Sheet

| Symbol | Meaning | Example |
|--------|---------|---------|
| **[[a; b]]** | Discrete interval (integers) | [[0; 5]] = {0, 1, 2, 3, 4, 5} |
| **[a; b]** | Continuous interval (reals) | [0; 1] = {all reals from 0 to 1} |
| **A²** | Cartesian product A × A | ℕ² = ℕ × ℕ = pairs of naturals |
| **⊂** | Subset (contained in) | {1, 2} ⊂ ℕ |
| **∈** | Element of (belongs to) | 5 ∈ ℕ |
| **→** | Maps to | f: A → B |
| **ℕ** | Natural numbers | {0, 1, 2, 3, ...} |
| **ℝ** | Real numbers | All numbers on number line |

---

### Common Variations

**Discrete 2D → Discrete 1D:**
```
f : (x, y) ∈ [[0; 2¹⁶ - 1]]² → [[0; 2³² - 1]]

// Your typical Ex10 implementation
pub fn map(x: u16, y: u16) -> u32
```

**Continuous 1D → Continuous 2D:**
```
f : t ∈ [0; 1] → [0; 1]²

// Classical space-filling curve
fn space_filling(t: f64) -> (f64, f64)
```

**Discrete 1D → Discrete 2D:**
```
f⁻¹ : i ∈ [[0; 2³² - 1]] → [[0; 2¹⁶ - 1]]²

// Inverse of your map function
pub fn unmap(index: u32) -> (u16, u16)
```

---

### Why This Notation Matters

Understanding this notation helps you:

1. **Understand the problem domain:**
   - What types of inputs? (discrete integers vs continuous reals)
   - What range of inputs? (0 to 65535)
   - How many dimensions? (2D coordinate pairs)

2. **Design the solution:**
   - Output type? (real number between 0 and 1)
   - Bijection required? (depends on subset notation)
   - Continuous or discrete? (affects algorithm choice)

3. **Verify correctness:**
   - Are all inputs handled? (domain coverage)
   - Are outputs in range? (codomain membership)
   - Is mapping unique? (bijectivity)

For your Ex10, this notation formally specifies that you need a function mapping **all 4,294,967,296 integer coordinate pairs** to **real values between 0 and 1**! 🎯

## Ex10 Implementation: Choosing Your Space-Filling Curve

### What Does the Requirement Actually Say?

**From the subject:**
```
To satisfy the requirement that card(A) = 2^32,
we interleave the 16 bits of x and 16 bits of y into a single u32.
Then, we map that u32 into the range [0, 1].
```

**Translation:**

This is a **two-step process**:

1. **Step 1: Bit Interleaving** (Creates the bijection)
   - Combine 16 bits of x and 16 bits of y into a single u32
   - This creates the space-filling curve ordering
   - Result: 2^32 unique values (one for each grid point)

2. **Step 2: Normalization** (Maps to [0, 1])
   - Take the u32 result
   - Map it to a real number in [0, 1]
   - This is just scaling: divide by u32::MAX

---

### Do You Choose the Algorithm?

**Yes, with a caveat!**

The subject gives you **freedom** in choosing which space-filling curve to use:

```
✅ You CAN implement Peano curve
✅ You CAN implement Hilbert curve
✅ You CAN implement Z-Order (Lebesgue) curve
✅ You CAN invent your own bijective mapping
```

**The ONLY requirement is:**
- Your function must be **bijective** (card(A) = 2^32)
- You must normalize to [0, 1]

---

### Comparing the Three Curves

#### 1. **Z-Order Curve (Lebesgue / Morton Code)** ⭐ EASIEST

**How it works:**
```
Bit interleaving of x and y coordinates:

x = 0b 0101 0011 (binary)
y = 0b 1100 1010 (binary)

Interleave bits:
Result = 0b 11_01_10_00_10_11_00_10

Pattern: y₁₅ x₁₅ y₁₄ x₁₄ ... y₀ x₀
```

**Code example:**
```rust
fn map_z_order(x: u16, y: u16) -> f64 {
    let mut z: u32 = 0;
    
    // Interleave bits of x and y
    for i in 0..16 {
        z |= ((x as u32 >> i) & 1) << (2 * i);      // x bit at position 2*i
        z |= ((y as u32 >> i) & 1) << (2 * i + 1);  // y bit at position 2*i+1
    }
    
    // Normalize to [0, 1]
    z as f64 / u32::MAX as f64
}
```

**Pros:**
- ✅ Simple to understand
- ✅ Easy to implement
- ✅ Computationally fast (O(16) operations)
- ✅ Creates Z-pattern at each scale
- ✅ This is likely what the subject intends!

**Cons:**
- ❌ Locality not as good as Hilbert
- ❌ Some jumps between regions

**Visual Pattern:**
```
┌─────┬─────┐
│ ┌─┐ │ ┌─┐ │
│ │0│ │ │2│ │
│ └─┘ │ └─┘ │
├─────┼─────┤
│ ┌─┐ │ ┌─┐ │
│ │1│ │ │3│ │
│ └─┘ │ └─┘ │
└─────┴─────┘

Forms Z-shape at each level of subdivision
```

---

#### 2. **Hilbert Curve** ⭐⭐ BEST LOCALITY

**How it works:**
```
Recursive subdivision with 90-degree rotations

Level 1:        Level 2:         Level 3:
┌─┐            ┌─┬─┐           ┌─┬───┬─┐
│→│            │ ┌─┐ │         │ ├──┬┤ │
└─┘            └─┘ └─┘         └─┴──┴─┘

Each iteration rotates quadrants to maintain continuity
```

**Code example:**
```rust
fn map_hilbert(x: u16, y: u16, n: u16) -> f64 {
    let mut d: u32 = 0;
    let mut s = 1 << (n - 1);
    let mut x = x as u32;
    let mut y = y as u32;
    
    while s > 0 {
        let rx = ((x & s) > 0) as u32;
        let ry = ((y & s) > 0) as u32;
        d += s * s * ((3 * rx) ^ ry);
        
        // Rotate coordinates
        if ry == 0 {
            if rx == 1 {
                x = s - 1 - x;
                y = s - 1 - y;
            }
            std::mem::swap(&mut x, &mut y);
        }
        s >>= 1;
    }
    
    d as f64 / ((1u32 << (2 * n)) - 1) as f64
}
```

**Pros:**
- ✅ Best locality preservation
- ✅ Nearby points in grid are nearby on curve
- ✅ Great for cache performance
- ✅ Most "natural" ordering

**Cons:**
- ❌ More complex to implement
- ❌ Requires rotation logic
- ❌ Needs parameter n (iterations/order)
- ❌ Slower than Z-order (O(log n) operations)

**Visual Pattern:**
```
Each quadrant connects smoothly via rotations
┌───────┐
│1  2   │
│  ╱╲   │
│0╱  ╲3 │
└───────┘

Maintains locality across all scales
```

---

#### 3. **Peano Curve** ⭐⭐ FIRST DISCOVERED

**How it works:**
```
9-fold division (3×3 grid instead of 2×2)

More chaotic than Hilbert
Fills space in a more "jumpy" way
```

**Code example:**
```rust
fn map_peano(x: u16, y: u16) -> f64 {
    let mut d: u32 = 0;
    let mut power = 1;
    let mut x = x as u32;
    let mut y = y as u32;
    
    while power < (1u32 << 16) {
        let (dx, dy) = peano_position(x / power, y / power);
        d += (dx * 3 + dy) * power * power;
        power *= 3;
    }
    
    d as f64 / ((1u32 << 32) - 1) as f64
}

fn peano_position(x: u32, y: u32) -> (u32, u32) {
    // Complex mapping for 3×3 grid positions
    // ...
}
```

**Pros:**
- ✅ Historical significance
- ✅ First proved continuous bijection existed

**Cons:**
- ❌ Much more complex to implement
- ❌ Works on 3×3 (doesn't map cleanly to binary)
- ❌ No inherent 16-bit structure
- ❌ Poor locality
- ❌ Not recommended for this problem!

**Recommendation:** Don't use this for Ex10

---

### What the Subject Actually Recommends

Looking at the subject's hint:

```
"To satisfy the requirement that card(A) = 2^32,
we interleave the 16 bits of x and 16 bits of y into a single u32."
```

**This is describing the Z-Order curve!**

The mention of "interleaving bits" is a dead giveaway:
- Z-Order = bit interleaving
- Hilbert = requires rotations
- Peano = requires 3×3 logic

---

### Recommendation: Use Z-Order Curve

**For Ex10, Z-Order is the best choice because:**

1. ✅ **Matches the subject description** ("interleave 16 bits of x and 16 bits of y")
2. ✅ **Simple to implement** (1-2 lines of code vs 20+ for Hilbert)
3. ✅ **Correct cardinality** (produces exactly 2^32 unique values)
4. ✅ **Efficient** (fast computation, no recursion)
5. ✅ **Still a valid space-filling curve** (bijective, with reasonable locality)

---

### Implementation Strategy

**Step 1: Interleave Bits (Z-Order)**

```rust
pub fn map(x: u16, y: u16) -> f64 {
    let mut z: u32 = 0;
    
    // Interleave 16 bits from x and y
    for i in 0..16 {
        z |= ((x as u32 >> i) & 1) << (2 * i);      // x bit at even positions
        z |= ((y as u32 >> i) & 1) << (2 * i + 1);  // y bit at odd positions
    }
    
    // Normalize to [0, 1]
    z as f64 / u32::MAX as f64
}
```

**Step 2: Verify Cardinality**

```rust
#[test]
fn test_cardinality() {
    // Every (x, y) should produce a unique value
    let mut seen = std::collections::HashSet::new();
    
    for x in 0..256 {  // Test subset
        for y in 0..256 {
            let value = map(x as u16, y as u16);
            assert!(!seen.contains(&value.to_bits()), "Collision detected!");
            seen.insert(value.to_bits());
        }
    }
    
    // Should have seen 256 * 256 unique values
    assert_eq!(seen.len(), 256 * 256);
}
```

**Step 3: Verify Range**

```rust
#[test]
fn test_range() {
    // All outputs should be in [0, 1]
    for x in 0..=u16::MAX {
        for y in 0..=u16::MAX {
            let value = map(x, y);
            assert!(value >= 0.0 && value <= 1.0);
        }
    }
}
```

---

### Can You Do Better?

**If you want to implement Hilbert for better locality:**

```rust
pub fn map_hilbert(x: u16, y: u16) -> f64 {
    let n = 16;  // For u16 × u16
    let mut d: u32 = 0;
    let mut s = 1u32 << (n - 1);
    let mut x = x as u32;
    let mut y = y as u32;
    
    while s > 0 {
        let rx = ((x & s) > 0) as u32;
        let ry = ((y & s) > 0) as u32;
        d += s * s * ((3 * rx) ^ ry);
        
        if ry == 0 {
            if rx == 1 {
                x = s - 1 - x;
                y = s - 1 - y;
            }
            std::mem::swap(&mut x, &mut y);
        }
        s >>= 1;
    }
    
    // Normalize to [0, 1]
    d as f64 / ((1u64 << 32) - 1) as f64
}
```

**Pros of implementing Hilbert:**
- ✅ Shows deeper understanding
- ✅ Better spatial locality
- ✅ Impressive in code review
- ✅ More elegant mathematically

**Cons:**
- ❌ More complex code
- ❌ Harder to debug
- ❌ Not explicitly required

---

### Summary: What You Should Do

| Aspect | Z-Order | Hilbert |
|--------|---------|---------|
| **Matches subject** | ✅ YES | ❌ NO |
| **Difficulty** | ⭐ Easy | ⭐⭐⭐ Hard |
| **Performance** | ⭐⭐⭐ Fast | ⭐⭐ Slower |
| **Code length** | ~10 lines | ~30 lines |
| **Locality** | Good | Excellent |
| **Recommended** | ✅ YES | ⭐ Optional |

---

### Final Recommendation

**Implement Z-Order Curve because:**

1. It matches the subject description ("interleave bits")
2. It's simple and efficient
3. It satisfies all requirements
4. It's the "obvious" choice for this problem

**But if you want to go further:**

Implement both Z-Order and Hilbert, and compare:
```rust
pub fn map_z_order(x: u16, y: u16) -> f64 { ... }
pub fn map_hilbert(x: u16, y: u16) -> f64 { ... }
pub fn map(x: u16, y: u16) -> f64 {
    // Use Z-order as default
    map_z_order(x, y)
}
```

This shows you understand the space of possible solutions! 🎯

## Z-Order Curve: Why It's Called "Z-Order"

### The Z Pattern Visualization

You're correct to question this! Let me show you the **actual Z pattern** at different scales:

### Level 1: 2×2 Grid

```
┌─────┬─────┐
│  0  │  2  │  The curve visits:
│     │     │  0 → 1 → 2 → 3
├─────┼─────┤
│  1  │  3  │  This makes a "Z" shape:
│     │     │  
└─────┴─────┘
     
     0 ──→ 2
     ↓     ↑
     1 ──→ 3
```

That's not quite a Z either, it's more like this:

```
0 ───→ 2
      ╱
     ╱
1 ───→ 3

Actually draws an "N" rotated 90° clockwise!
```

### Level 2: 4×4 Grid

```
┌────┬────┬────┬────┐
│  0 │  1 │  4 │  5 │
├────┼────┼────┼────┤
│  2 │  3 │  6 │  7 │
├────┼────┼────┼────┤
│  8 │  9 │ 12 │ 13 │
├────┼────┼────┼────┤
│ 10 │ 11 │ 14 │ 15 │
└────┴────┴────┴────┘

The path:
0→1    4→5
 ↓↑     ↓↑
2→3    6→7

8→9   12→13
 ↓↑     ↓↑
10→11  14→15
```

### Why "Z-Order" is a Misnomer

You're absolutely correct! The pattern actually looks more like an **"N"** or a **rotated "Z"** depending on how you look at it.

**The truth is:**

1. **Historical naming:** The name "Z-order" comes from the observation that at certain scales, the curve makes a shape **somewhat resembling a Z**, but it's not a perfect Z.

2. **Morton's naming:** It's also called "Morton code" after G.M. Morton who invented it in 1966. That's actually a more accurate name!

3. **The actual shape:** At the smallest 2×2 scale, it looks more like:
   ```
   Start → 2
   ↓       ↑
   1   →   End
   
   Which is closer to an "N" rotated!
   ```

### More Accurate Description

The Z-order curve is better described as:

**"A recursive space-filling curve that subdivides space into quadrants and visits them in a specific order that creates a fractal pattern through bit-interleaving."**

### The Real Z-Pattern (If We Squint)

At higher levels, you can kind of see a Z if you look at the **overall flow**:

```
Level 3 (8×8) - Showing just the flow:

Top-left quadrant    →    Top-right quadrant
       ↓                          ↓
       ↓                          ↓
       ↓                          ↓
Bottom-left quadrant  →  Bottom-right quadrant

This overall flow makes a "Z":
TL ──→ TR
 ↓
BL ──→ BR
```

But within each quadrant, it's the same recursive N/rotated-Z pattern!

### Alternative Name: "Morton Code"

Many people prefer calling it **Morton code** or **Morton order** because:
- ✅ Named after the inventor
- ✅ No confusion about the shape
- ✅ More technically accurate
- ✅ Emphasizes it's an encoding/indexing scheme

### The Bit-Interleaving Pattern Creates the Curve

The **real reason** for the pattern is the bit interleaving:

```
For coordinates (x, y):
x = 0b ...x₂ x₁ x₀
y = 0b ...y₂ y₁ y₀

Morton code = ...y₂ x₂ y₁ x₁ y₀ x₀

This interleaving naturally creates the recursive pattern!
```

**Example:**
```
(0,0) = x=0b00, y=0b00 → 0b0000 = 0
(1,0) = x=0b01, y=0b00 → 0b0001 = 1
(0,1) = x=0b00, y=0b01 → 0b0010 = 2
(1,1) = x=0b01, y=0b01 → 0b0011 = 3

The order 0→1→2→3 makes that N/rotated-Z shape!
```

### Summary: You're Right!

**You are correct:**
- The pattern looks more like an **"N"** (or rotated/mirrored Z)
- The name "Z-order" is somewhat misleading
- It's more accurately called **"Morton code"**
- The pattern comes from bit interleaving, not from drawing a Z

**Why we still call it Z-order:**
- Historical convention
- Widely used term in computer graphics
- "Sounds cooler" than "N-order curve" 😄
- At macro scale, the overall flow vaguely resembles a Z

**For your implementation:**
- The name doesn't matter—the algorithm is the same!
- Bit interleaving creates the pattern
- It's still a valid space-filling curve
- It still satisfies card(A) = 2³²

Great observation! Mathematical naming isn't always perfectly accurate. 🎯

## Z-Order Curve: The REAL Z Pattern

### You're Correct: It Depends on Bit Order!

**If we interleave as (x, y):** We get an "N" pattern
**If we interleave as (y, x):** We get a true "Z" pattern!

Let me show both:

---

### Pattern 1: (x, y) Interleaving → "N" Pattern

```
Interleaving pattern: x₀ y₀ x₁ y₁ x₂ y₂ ...
Result bits at positions: [... y₃ x₃ y₂ x₂ y₁ x₁ y₀ x₀]
                              odd even odd even odd even odd even
```

**Creates this ordering in 2×2 grid:**
```
┌─────┬─────┐
│  0  │  2  │   0 ──→ 2
│     │     │        ╱
├─────┼─────┤       ╱
│  1  │  3  │   1 ──→ 3
└─────┴─────┘
```

This makes an **"N" shape** (or rotated Z)!

---

### Pattern 2: (y, x) Interleaving → TRUE "Z" Pattern ✓

```
Interleaving pattern: y₀ x₀ y₁ x₁ y₂ x₂ ...
Result bits at positions: [... x₃ y₃ x₂ y₂ x₁ y₁ x₀ y₀]
                              even odd even odd even odd even odd
```

**Creates this ordering in 2×2 grid:**
```
┌─────┬─────┐
│  0  │  1  │   0 ──→ 1
│     │     │   ↓       ↘
├─────┼─────┤   ↓         ↘
│  2  │  3  │   2 ──────→ 3
└─────┴─────┘
```

This makes a **TRUE "Z" shape**! ✓

---

### Your Example Corrected

**Original data:**
```
x = 0b 0101_0011
y = 0b 1100_1010
```

### Option A: (x, y) Interleaving - "N" Pattern

```
Pattern: x₀ y₀ x₁ y₁ x₂ y₂ x₃ y₃ ...

Bit positions:
  i:   7  6  5  4  3  2  1  0
  x:   0  1  0  1  0  0  1  1
  y:   1  1  0  0  1  0  1  0

Result = x₀ y₀ x₁ y₁ x₂ y₂ x₃ y₃ x₄ y₄ x₅ y₅ x₆ y₆ x₇ y₇
       =  1  0  1  1  0  0  1  0  0  1  1  0  1  1  0  1

Result = 0b 01_11_10_01_10_00_11_10
       = 0b 0111100110001110 (binary)
```

### Option B: (y, x) Interleaving - TRUE "Z" Pattern ✓

```
Pattern: y₀ x₀ y₁ x₁ y₂ x₂ y₃ x₃ ...

Bit positions:
  i:   7  6  5  4  3  2  1  0
  x:   0  1  0  1  0  0  1  1
  y:   1  1  0  0  1  0  1  0

Result = y₀ x₀ y₁ x₁ y₂ x₂ y₃ x₃ y₄ x₄ y₅ x₅ y₆ x₆ y₇ x₇
       =  0  1  1  1  0  0  0  1  1  0  0  1  0  1  1  0

Result = 0b 10_11_01_00_11_00_11_01
       = 0b 1011010011001101 (binary)
```

---

### Updated Code for TRUE Z-Pattern

```rust
pub fn map_z_order(x: u16, y: u16) -> f64 {
    let mut z: u32 = 0;
    
    // Interleave bits: y at even positions, x at odd positions
    // This creates the TRUE "Z" pattern!
    for i in 0..16 {
        let x_bit = (x >> i) & 1;
        let y_bit = (y >> i) & 1;
        
        z |= (y_bit as u32) << (2 * i);      // y_bit at position 2*i (even)
        z |= (x_bit as u32) << (2 * i + 1);  // x_bit at position 2*i+1 (odd)
    }
    
    // Normalize to [0, 1]
    z as f64 / u32::MAX as f64
}
```

**Or more concisely:**
```rust
pub fn map_z_order(x: u16, y: u16) -> f64 {
    let mut z: u32 = 0;
    
    for i in 0..16 {
        z |= ((y as u32 >> i) & 1) << (2 * i);      // y → even positions
        z |= ((x as u32 >> i) & 1) << (2 * i + 1);  // x → odd positions
    }
    
    z as f64 / u32::MAX as f64
}
```

---

### Visual Comparison at Scale

**Pattern with (x, y) - "N" shape:**
```
4×4 grid:
┌────┬────┬────┬────┐
│  0 │  2 │  8 │ 10 │     0→2   8→10
│    │    │    │    │      ↓↑    ↓↑
├────┼────┼────┼────┤     1→3   9→11
│  1 │  3 │  9 │ 11 │
│    │    │    │    │     4→6  12→14
├────┼────┼────┼────┤      ↓↑    ↓↑
│  4 │  6 │ 12 │ 14 │     5→7  13→15
│    │    │    │    │
├────┼────┼────┼────┤  Makes "N" shapes
│  5 │  7 │ 13 │ 15 │
└────┴────┴────┴────┘
```

**Pattern with (y, x) - TRUE "Z" shape:**
```
4×4 grid:
┌────┬────┬────┬────┐
│  0 │  1 │  4 │  5 │     0→1   4→5
│    │    │    │    │     ↓  ↘  ↓  ↘
├────┼────┼────┼────┤     2─→3  6─→7
│  2 │  3 │  6 │  7 │
│    │    │    │    │     8→9  12→13
├────┼────┼────┼────┤     ↓  ↘  ↓  ↘
│  8 │  9 │ 12 │ 13 │    10→11 14→15
│    │    │    │    │
├────┼────┼────┼────┤  Makes TRUE "Z" shapes!
│ 10 │ 11 │ 14 │ 15 │
└────┴────┴────┴────┘
```

---

### Which One Should You Use?

**Both are valid space-filling curves!** The choice depends on convention:

| Aspect | (x, y) Interleaving | (y, x) Interleaving |
|--------|---------------------|---------------------|
| **Pattern** | "N" shape | TRUE "Z" shape |
| **Convention** | More common in literature | Makes name accurate |
| **Bijectivity** | ✅ YES (2³²) | ✅ YES (2³²) |
| **Locality** | Good | Good (same) |
| **Name accuracy** | Misleading | Accurate! |

**Most implementations use (x, y) order** because:
- It matches the conventional coordinate order
- It's what most papers and books use
- The name "Z-order" is historical anyway

**But (y, x) order is totally valid** and actually makes the name make sense!

---

### The Real Answer: Both Work!

For your Ex10, **either pattern satisfies the requirements:**

```
✅ Bijective: card(A) = 2³²
✅ Maps to [0, 1]
✅ Computable via bit interleaving
✅ Valid space-filling curve
```

The only difference is the **visual pattern** of how you traverse the grid.

---

### My Recommendation

**Use (x, y) order** (the "N" pattern) because:
1. ✅ Matches standard Morton code implementations
2. ✅ More common in graphics/database literature
3. ✅ Easier to find reference implementations
4. ✅ Conventional coordinate order (x, y)

But if you want to implement the **TRUE Z-pattern**, use (y, x) order—it's equally valid and makes the name accurate!

---

### Summary

**You discovered something important:**
- The "Z" vs "N" pattern depends on bit interleaving order
- (x, y) → "N" pattern (conventional)
- (y, x) → TRUE "Z" pattern (name-accurate)
- Both are valid bijective space-filling curves
- The choice is mostly aesthetic/conventional

Excellent observation! This is the kind of detail that shows deep understanding. 🎯

To show a "jump," you want to find a spot where the Z-curve has to travel all the way across the grid to start the next section. These jumps occur at the boundaries of power-of-two squares.

For example, when you move from  to , you are crossing a major binary threshold.

### 1. The "Locality Jump" Test

Add this to your `main` to show that while the curve is mostly local, it isn't perfect (unlike the Hilbert curve):

```rust
println!("Demonstrating a Z-Curve 'Jump':");
let a = map(127, 127);
let b = map(128, 127); // Only 1 pixel away in 2D space!

println!("Point A (127, 127): {:.15}", a);
println!("Point B (128, 127): {:.15}", b);
println!("Distance in 1D:      {:.15}", (a - b).abs());
println!("Note: A small 1-pixel step in 2D caused a massive jump in 1D!");

```

### 2. Final Logic Polish

Just to make sure your internal logic is crystal clear for your README or defense:

* **Interleaving  at  and  at :** This means  is the "Least Significant" part of each bit-pair. Therefore, moving horizontally () changes the -value **faster** than moving vertically ().
* **Result:** You get a horizontal "Z".

If you swapped them ( at ), moving vertically would change the -value faster, and you'd get a vertical "N".

### 3. Summary for your Defense

If an evaluator asks why you chose the Z-order curve:

1. **Efficiency:** It uses simple bitwise shifts and OR operations ( where  is bit depth).
2. **Cardinality:** It is a perfect **Bijection**. Since every bit of the input is preserved in a unique position in the output, no two coordinates can ever map to the same float.
3. **Morphism:** It preserves the "structure" of the binary data while transforming the dimensionality.

Exercise 10: Curve Saturation — The Grand Finale
The Goal

The objective was to create a morphism (a structure-preserving map) from a 2D discrete space into a 1D continuous space:

f:(x,y)∈[[0;2 
16
 −1]] 
2
 →[0;1]⊂R
Why the Z-Order Curve?

I chose the Z-order curve (also known as the Morton Curve) because it provides a perfect bijection. By interleaving the bits of the x and y coordinates, we ensure that:

Cardinality is preserved: Every unique (x,y) coordinate maps to exactly one unique value on the number line. Since there are 2 
16
 ×2 
16
  possible coordinates, there are exactly 2 
32
  unique outputs.

Locality is maintained: Points that are close together in 2D space generally stay relatively close together in 1D space, which is a key property of space-filling curves.

How it Works (The Bit Weaving)

To map the coordinates, the bits are "interleaved" like teeth on a zipper:

Input: Two 16-bit integers (x and y).

Processing:

Bits of x are moved to even positions (0,2,4…30).

Bits of y are moved to odd positions (1,3,5…31).

Result: A 32-bit integer that represents the "Morton code" or Z-index.

Mathematical Significance

This exercise demonstrates the transition from Discrete Mathematics (counting bits and sets) to Continuous Mathematics (mapping to the Real number line R). By dividing the final integer by 2 
32
 −1, we "saturate" the unit interval [0,1], proving that we can represent multi-dimensional data in a single dimension without losing information.

 ## Ex11: Inverse Function and Function Composition

### What is an Inverse Function?

An **inverse function** f⁻¹ "undoes" what f does. If f maps from A to B, then f⁻¹ maps from B back to A.

**Notation:**
```
f : A → B     (forward function)
f⁻¹ : B → A   (inverse function)
```

---

### The Two Composition Laws

Your exercise requires two properties to hold:

```
(f⁻¹ ∘ f)(x, y) = (x, y)
(f ∘ f⁻¹)(x) = x
```

**These express the same idea in two directions!** Let me explain each.

---

### Law 1: (f⁻¹ ∘ f)(x, y) = (x, y)

**Read as:** "f inverse composed with f equals the identity"

**Breaking it down:**

```
(f⁻¹ ∘ f)(x, y) means: f⁻¹(f(x, y))
                       └──┬──┘  └─┬─┘
                         Apply f first
                              Then apply f⁻¹
```

**Step by step:**

```
Input: (x, y) ∈ [[0; 2¹⁶ - 1]]²

Step 1: Apply f
        f(x, y) = some_value ∈ [0, 1]
        Result: a single float

Step 2: Apply f⁻¹
        f⁻¹(some_value) = ?
        Result: back to (x, y)

Expected: (x, y)  ← We got back what we started with!
```

**Why this works:**

Since f is a **bijection** (one-to-one, onto), every point in [0, 1] came from exactly one (x, y) pair. So when you apply f⁻¹ to that point, it must return to the original (x, y).

**Example:**
```
f(100, 200) = 0.001234567  (some value in [0, 1])
f⁻¹(0.001234567) = (100, 200)  ← Back to the original!

So: (f⁻¹ ∘ f)(100, 200) = f⁻¹(f(100, 200)) = f⁻¹(0.001234567) = (100, 200) ✓
```

---

### Law 2: (f ∘ f⁻¹)(x) = x

**Read as:** "f composed with f inverse equals the identity"

**Breaking it down:**

```
(f ∘ f⁻¹)(x) means: f(f⁻¹(x))
                    └───┬───┘  └┬┘
                    Apply f⁻¹ first
                          Then apply f
```

**Step by step:**

```
Input: x ∈ [0, 1]

Step 1: Apply f⁻¹
        f⁻¹(x) = some (a, b) ∈ [[0; 2¹⁶ - 1]]²
        Result: a 2D coordinate pair

Step 2: Apply f
        f(a, b) = ?
        Result: back to a float

Expected: x  ← We got back what we started with!
```

**Why this works:**

Since f⁻¹ is the inverse, it maps [0, 1] back to [[0; 2¹⁶ - 1]]². When you then apply f to that coordinate pair, it must return the original float value.

**Example:**
```
f⁻¹(0.001234567) = (100, 200)
f(100, 200) = 0.001234567  ← Back to the original!

So: (f ∘ f⁻¹)(0.001234567) = f(f⁻¹(0.001234567)) = f(100, 200) = 0.001234567 ✓
```

---

### Why Both Laws Express the Same Thing

**They are complementary:**

| Law | Direction | Meaning |
|-----|-----------|---------|
| **(f⁻¹ ∘ f)** | 2D → 1D → 2D | "Going forward then backward gets you home" |
| **(f ∘ f⁻¹)** | 1D → 2D → 1D | "Going backward then forward gets you home" |

**Both express the same mathematical truth:** f and f⁻¹ are **perfect inverses** of each other.

Think of it like a door:

```
Room A                      Room B
(2D space)                  (1D space)
[[0; 2¹⁶-1]]²            [0, 1]

  (x, y) ───f──→ value
                    ↓
                  f⁻¹
                    ↓
              (x, y) again ✓

  value ───f⁻¹──→ (x, y)
               ↓
              f
               ↓
          value again ✓
```

---

### Formal Definition: Function Composition

**Composition symbol ∘** means "apply the right function first, then the left"

```
(g ∘ f)(x) = g(f(x))
             └──┬──┘  Apply f first
              Then g
```

**Important:** The order matters!
```
(g ∘ f)(x) ≠ (f ∘ g)(x)  in general
```

---

### The Identity Function

Both laws mention "the identity":

```
Identity on [[0; 2¹⁶-1]]²:
id₂D(x, y) = (x, y)        ← Returns the same pair

Identity on [0, 1]:
id₁D(x) = x               ← Returns the same value
```

**The laws say:**
```
f⁻¹ ∘ f = id₂D   (2D identity)
f ∘ f⁻¹ = id₁D   (1D identity)
```

This is the **definition of an inverse function**!

---

### Implementing the Inverse (Ex11)

For your Z-order curve, you need to implement the inverse:

```rust
pub fn unmap(z: f64) -> (u16, u16) {
    // Step 1: Convert from [0, 1] back to u32
    let index = (z * u32::MAX as f64) as u32;
    
    // Step 2: De-interleave bits
    // Extract x from even positions: 0, 2, 4, ...
    let mut x: u16 = 0;
    for i in 0..16 {
        x |= ((index >> (2 * i)) & 1) as u16;
        x <<= 1;
    }
    
    // Extract y from odd positions: 1, 3, 5, ...
    let mut y: u16 = 0;
    for i in 0..16 {
        y |= ((index >> (2 * i + 1)) & 1) as u16;
        y <<= 1;
    }
    
    (x, y)
}
```

**OR more cleanly:**

```rust
pub fn unmap(z: f64) -> (u16, u16) {
    let index = (z * u32::MAX as f64) as u32;
    
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    
    for i in 0..16 {
        x |= ((index >> (2 * i)) & 1) as u16 << i;      // Even positions → x bits
        y |= ((index >> (2 * i + 1)) & 1) as u16 << i;  // Odd positions → y bits
    }
    
    (x, y)
}
```

---

### Testing the Inverse

**Test both composition laws:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f_inverse_f() {
        // (f⁻¹ ∘ f)(x, y) = (x, y)
        let test_points = vec![
            (0, 0),
            (100, 200),
            (u16::MAX, u16::MAX),
            (12345, 54321),
        ];
        
        for (x, y) in test_points {
            let z = map(x, y);              // Apply f
            let (x_recovered, y_recovered) = unmap(z);  // Apply f⁻¹
            
            assert_eq!((x_recovered, y_recovered), (x, y),
                      "Failed for ({}, {})", x, y);
        }
    }

    #[test]
    fn test_f_of_f_inverse() {
        // (f ∘ f⁻¹)(x) = x
        let test_values = vec![
            0.0,
            0.5,
            1.0,
            0.001234567,
            0.999999999,
        ];
        
        for z in test_values {
            let (x, y) = unmap(z);           // Apply f⁻¹
            let z_recovered = map(x, y);     // Apply f
            
            // Use approximate equality for floats
            assert!((z_recovered - z).abs() < 1e-10,
                   "Failed for {}", z);
        }
    }

    #[test]
    fn test_bijectivity() {
        // If they're true inverses, they should be bijective
        use std::collections::HashSet;
        let mut forward_values = HashSet::new();
        let mut reverse_coords = HashSet::new();
        
        for x in 0..256 {
            for y in 0..256 {
                let z = map(x as u16, y as u16);
                forward_values.insert(z.to_bits());
                
                let (x_back, y_back) = unmap(z);
                reverse_coords.insert((x_back, y_back));
            }
        }
        
        // All forward mappings should be unique
        assert_eq!(forward_values.len(), 256 * 256);
        // All reverse mappings should be unique
        assert_eq!(reverse_coords.len(), 256 * 256);
    }
}
```

---

### Why "x alone" Works Too

You asked: **"does it work for x alone too?"**

**YES! Absolutely!** Here's why:

```
For Law 2: (f ∘ f⁻¹)(x) = x

This takes a SINGLE value x and shows that:
f⁻¹(x) produces a coordinate pair (a, b)
Then f(a, b) produces back the original x

So yes, this law works entirely with a single 1D value!
```

**The key insight:**

- **Law 1** shows: 2D → 1D → 2D returns to 2D
- **Law 2** shows: 1D → 2D → 1D returns to 1D

They're testing the **round trip in both directions**!

---

### Visual Summary

```
Forward (f):
┌─────────────────────────┐
│ (x, y) ────f───→ z ∈ [0,1]
│                         
│ 2D Grid            1D Line
└─────────────────────────┘

Reverse (f⁻¹):
┌─────────────────────────┐
│ z ∈ [0,1] ──f⁻¹──→ (x, y)
│                         
│ 1D Line            2D Grid
└─────────────────────────┘

Composition 1 (2D → 1D → 2D):
┌──────────────────────────────┐
│ (x,y) ─f─→ z ─f⁻¹─→ (x,y)   │
│  "What we started with" = "What we got"
└──────────────────────────────┘

Composition 2 (1D → 2D → 1D):
┌──────────────────────────────┐
│ z ──f⁻¹─→ (x,y) ─f─→ z       │
│  "What we started with" = "What we got"
└──────────────────────────────┘
```

---

### Mathematical Terminology

**These laws define what it means to be an inverse:**

```
A function f⁻¹ is the inverse of f if and only if:

1. f⁻¹ ∘ f = id_domain      (Going forward then backward is identity)
2. f ∘ f⁻¹ = id_codomain    (Going backward then forward is identity)
```

**This is the DEFINITION of an inverse function in set theory and category theory!**

---

### Connection to Your Project

**Ex10 → Ex11 Arc:**

| Exercise | Task | Result |
|----------|------|--------|
| **Ex10** | Implement f (forward) | 2D → 1D bijection |
| **Ex11** | Implement f⁻¹ (inverse) | 1D → 2D bijection |
| **Property** | (f⁻¹ ∘ f) = id | Perfect round-trip both ways |

You're building an **isomorphism** between spaces!

```
In Category Theory terms:
[[0; 2¹⁶-1]]² ≅ [0, 1]

(The ≅ symbol means "isomorphic to")
```

This means the two spaces have the **same structure**, just different representations.

---

### Why This Matters

**In Computer Science:**

1. **Data Serialization:** Convert multi-dimensional data to 1D for storage
   ```
   (x, y) coordinates → single float (Ex10)
   Read from disk → recover (x, y) (Ex11)
   ```

2. **Hash Functions:** Map objects to indices
   ```
   grid coordinates → array index
   array index → grid coordinates
   ```

3. **Spatial Indexing:** Organize 2D data on 1D structures
   ```
   B-trees, databases use space-filling curves
   Store 2D geographic data in 1D index
   ```

4. **Morphisms:** Prove two structures are equivalent
   ```
   "These two spaces are the same algebraically"
   Proven by showing bijective morphism
   ```

---

### Summary: The Two Laws

**You have it exactly right:**

```
(f⁻¹ ∘ f)(x, y) = (x, y)     ← "I can reverse the function and get back!"
(f ∘ f⁻¹)(x) = x              ← "And it works going the other way too!"
```

Both express the same mathematical truth:
- **f and f⁻¹ are perfect inverses**
- **They form a bijection (one-to-one, onto)**
- **Together they preserve all information**

This is the pinnacle of your Boolean algebra journey! From sets and morphisms → to space-filling curves → to inverse functions. You've built up from pure theory to practical bijective mappings. 🎯


Q:
so it seems the f64 type is enough to represent the values. out of curiosity how many values can f64 represent in the space 0..1 included?
A:
About 9,007,199,254,740,993 distinct values in [0, 1] (i.e., 2⁵³ + 1).
That’s because f64 has 53 bits of precision, so it can represent multiples of 2⁻⁵³ near 1, plus 0.

So yes: f64 has far more than the 2³² values you need. f32 has only 24 bits of precision (~2²⁴ + 1 values in [0,1]), which is far less than 2³². So f32 cannot uniquely represent all your mapped values.