# Group Theory

## Set vs Group: The Fundamental Difference

### What is a **Set**?

A **set** is simply a collection of distinct elements with **no structure**.

**Examples:**

- {1, 2, 3} - just three numbers
- {apple, banana, orange} - just three fruits
- {A, B, C} - just three variables

**Properties of sets:**

- ✅ Elements are distinct (no duplicates)
- ✅ Order doesn't matter {1, 2, 3} = {3, 2, 1}
- ✅ No operations defined
- ✅ No relationships between elements

**Set operations (add structure):**

- Union: A ∪ B
- Intersection: A ∩ B
- Complement: A'

But these are just **actions on sets**, not a "group structure."

---

### What is a **Group**?

A **group** is a set **PLUS** a binary operation that satisfies 4 special properties.

**Definition:** A group (G, ∘) consists of:

1. A set G of elements
2. A binary operation ∘ (some way to combine two elements)

**The 4 Group Axioms:**

| Property | Meaning | Example (ℤ, +) |
| -------- | ------- | --------------- |
| **Closure** | Combining any two elements gives another element in the group | 3 + 5 = 8 ∈ ℤ ✓ |
| **Associativity** | (a ∘ b) ∘ c = a ∘ (b ∘ c) | (1 + 2) + 3 = 1 + (2 + 3) ✓ |
| **Identity Element** | There exists an element e where a ∘ e = a for all a | 0 + 5 = 5 ✓ |
| **Inverse Element** | For each element a, there exists a⁻¹ where a ∘ a⁻¹ = e | 5 + (-5) = 0 ✓ |

---

### Examples: Set vs Group

#### Example 1: Natural Numbers

```txt
Set: ℕ = {0, 1, 2, 3, ...}  (just elements, no operation)

Group: (ℕ, +) = Natural numbers WITH addition
  - Closure: 3 + 5 = 8 ✓
  - Associativity: (1+2)+3 = 1+(2+3) ✓
  - Identity: 0 (because 5 + 0 = 5) ✓
  - Inverse: FAIL! ✗ (no negative numbers in ℕ)
  
So (ℕ, +) is NOT a group, but (ℤ, +) IS a group!
```

#### Example 2: Boolean Values

```txt
Set: {0, 1}  (just true/false, no operation)

Group: ({0, 1}, XOR) = Boolean values WITH XOR operation
  - Closure: 0 XOR 1 = 1 ✓
  - Associativity: (0⊕1)⊕1 = 0⊕(1⊕1) ✓
  - Identity: 0 (because 1 ⊕ 0 = 1) ✓
  - Inverse: Each element is its own inverse (1⊕1 = 0) ✓
  
So ({0, 1}, XOR) IS a group!
```

#### Example 3: Rotation of a Square

```
Set: {0°, 90°, 180°, 270°}  (just angles)

Group: ({0°, 90°, 180°, 270°}, rotation)
  - Closure: 90° + 180° = 270° ✓
  - Associativity: (90° + 90°) + 90° = 90° + (90° + 90°) ✓
  - Identity: 0° (no rotation) ✓
  - Inverse: 90° ⟷ 270° (they cancel out) ✓
  
This is the famous "Cyclic Group of Order 4"!
```

---

### Quick Comparison Table

| Concept | Set | Group |
| --------- | ----- | ------- |
| **Definition** | Collection of elements | Set + binary operation |
| **Structure** | No relationships | Elements can be combined |
| **Example** | {apple, banana, cherry} | (ℤ, +) integers with addition |
| **Requirements** | None | 4 axioms must hold |
| **Can be empty?** | Yes | No (must have identity) |
| **Operation needed?** | No | Yes, exactly one |

---

### Connection to The Boolean Algebra Project

**In the exercises:**

- **Sets** (Ex08 Powerset): Just collecting all subsets {}, {A}, {B}, {A,B}...
- **Operations** (Ex00-Ex06): Defining ∧, ∨, ¬ on sets
- **Group structure**: ({0, 1}, XOR) is an abelian group!

**Why this matters:**

- Boolean algebra uses group theory under the hood
- XOR is a group operation (useful in cryptography)
- Understanding groups helps with circuit design and abstract algebra

---

### Why Groups are Powerful

Groups are the language of **symmetry** in mathematics:

- **Rotations** of objects form a group
- **Permutations** of elements form a group  
- **Symmetries** of crystals form a group
- **Molecular chemistry** uses group theory

They appear everywhere because any time we have a **reversible operation** (like addition, XOR, rotation), we likely have a group.

## The Order of a group

The Order of a group (often denoted as  or ) is simply the number of elements in the group's underlying set.

### 1. Finite vs. Infinite Orders

- **Finite Groups:** If a group has a specific number of elements, its order is that number.
- **Example:** The group of Boolean values  has an **order of 2**.
- **Example:** A 2x2 Rubik's Cube group has an order of **3,674,160**.

- **Infinite Groups:** If the set is something like the Integers (), we say the group has **infinite order**.

### 2. What is "Cardinality"?

The term **Cardinality** is used because it refers to the "size" of the set, regardless of how the elements are arranged or what the operation is.

In our project, the **Truth Table** for  variables has  rows. In a way, we are dealing with a Boolean space of **Order**.

---

### 3. Order of an Element (A different concept!)

We should be careful not to confuse the **Order of the Group** with the **Order of an Element**.

- The **Order of an Element**  is the smallest positive integer such that applying the operation n times brings us back to the **Identity** ().
-

In the XOR group :

- The **Order of the Group** is 2 (elements are  and ).
- The **Order of element 1** is 2 (because ).

---

### Ex10: Bringing it all together

For **Ex10 (Curve Saturation)**, we are working within a discrete coordinate space.

- If  and  are `u16`, each has  possible values.
- The "Space" of all possible  pairs is a grid with a "Cardinality" (Order) of .
- Our `map` function takes these  discrete points and maps them to  points on a 1D line.

Because the mapping is **Symmetrical** (Bijective), we are essentially rearranging the order of these  points without losing a single one.

## Morphisms: Structure-Preserving Maps

### What is a **Morphism**?

A **morphism** is a map (function) between two algebraic structures that **preserves their structure**.

The word comes from Greek: "morphe" = form/shape, "ism" = the study of.

A morphism is a "respectful translation" between two structures. It's not arbitrary—it must respect how the elements relate to each other.

---

### Example of a morphism

Imagine we have two groups:

- Group A with elements and operation ∘
- Group B with elements and operation ★

A morphism f: A → B is a function where:

```txt
f(a ∘ b) = f(a) ★ f(b)
```

It doesn't matter if we combine first then map, or map first then combine— we get the same result.

---

### Visual Example: The Integers

```txt
(ℤ, +)  ────────────→  (ℤ/5ℤ, +)
 
 Integer Addition    Modulo 5 Addition
 
   5 + 3 = 8              5 + 3 ≡ 3 (mod 5)
      ↓                         ↓
    8 mod 5 = 3              3 = 3 ✓
    
The function f(x) = x mod 5 is a MORPHISM because:
f(5 + 3) = f(8) = 3
f(5) + f(3) = 0 + 3 = 3  ✓ Same result!
```

---

### Types of Morphisms

| Type | Name | Definition | Example |
| ------ | ------ | ----------- | --------- |
| **Homomorphism** | Structure-preserving | f(a ∘ b) = f(a) ★ f(b) | f(x) = x mod 5 |
| **Isomorphism** | Bijective + structure-preserving | Homomorphism that's 1-to-1 and onto | f(x) = 2x (ℝ → ℝ) |
| **Endomorphism** | Maps to itself | Morphism f: G → G | f(x) = x² in (ℝ, ×) |
| **Automorphism** | Bijective endomorphism | Isomorphism f: G → G | f(x) = -x in (ℤ, +) |

---

### Example 1: Logarithm

```txt
GROUP A: (ℝ⁺, ×)        GROUP B: (ℝ, +)
Positive reals           All reals
with multiplication      with addition

f(x) = log(x)

Morphism property:
log(a × b) = log(a) + log(b) ✓

Example:
log(10 × 100) = log(1000) = 3
log(10) + log(100) = 1 + 2 = 3  ✓

This is an ISOMORPHISM because:
- Every positive number has a unique log (bijective)
- The structure is perfectly preserved
```

---

### Example 2: Boolean Values and XOR

```txt
GROUP A: ({0, 1}, XOR)      GROUP B: (ℤ/2ℤ, +)
Boolean XOR               Integers mod 2

f(0) = 0
f(1) = 1

Morphism property:
f(a ⊕ b) = f(a) + f(b) (mod 2)

Example:
1 ⊕ 1 = 0
f(0) = 0
f(1) + f(1) = 1 + 1 = 0 (mod 2)  ✓

This is an ISOMORPHISM because:
- It's one-to-one and onto (bijective)
- XOR in binary = addition mod 2
```

---

### Example 3: Determinant Function

```txt
GROUP A: (GL₂(ℝ), ×)              GROUP B: (ℝ*, ×)
2×2 invertible matrices           Non-zero real numbers
with multiplication               with multiplication

f(A) = det(A)

Morphism property:
det(A × B) = det(A) × det(B) ✓

Example:
det([[1, 2], [3, 4]]) = -2
det([[0, 1], [1, 0]]) = -1

det(A × B) = (-2) × (-1) = 2
det(A) × det(B) = (-2) × (-1) = 2  ✓

This is a HOMOMORPHISM but NOT an isomorphism
(not bijective—many matrices map to the same determinant)
```

---

### Morphisms in the real world

#### 1. Classification

- Morphisms help classify structures
- If two groups are isomorphic, they're "the same" algebraically

#### 2. Simplification

- Map a complex structure to a simpler one
- Solve the problem in the simpler space, then translate back

#### 3. Computer Science

- **Hashing** is a homomorphism (structure preserved but not bijective)
- **Encoding/Decoding** uses morphisms
- **Cryptography** exploits properties of morphisms

#### 4. Physics

- Symmetries of physical systems are automorphisms
- Conservation laws arise from morphisms

---

### Connection to the project

**Ex00-Ex03:** we define operations on {0, 1}

- These operations form group structures
- f(x) = NOT x is an automorphism of ({0, 1}, XOR)

**Ex05-Ex06:** Formula transformations (NNF, CNF)

- Converting between normal forms is a **morphism**
- The formula structure is preserved: f(A & B) = f(A) & f(B)

**Ex08:** Powerset

- The powerset operation is a **functor** (higher-level morphism)
- Maps sets to sets of subsets while preserving structure

**Ex09:** Set evaluation

- Our `eval_set` function is a homomorphism
- It maps formulas to their set-theoretic meaning
- f(A & B) = f(A) ∩ f(B)

---

### How To check if something is a morphism

1. Define two structures with operations
2. Pick a function f
3. Test: Does f(a ∘ b) = f(a) ★ f(b)?
4. If YES → it's a homomorphism
5. If YES + bijective → it's an isomorphism

**Types by "strength":**

```txt
Isomorphism (strongest)
    ↓
Homomorphism
    ↓
Regular function (weakest)
```

An isomorphism preserves **everything**. A homomorphism preserves only the **operation**. A regular function preserves nothing.

## Categories and Function Properties

### What is a **Category**?

A **category** is a collection of objects (like sets, groups, etc.) together with **morphisms** (functions/arrows) between them that follow certain rules.

#### Example: The Category "Set":

This is the most fundamental category in mathematics

- **Objects:** All sets
- **Morphisms:** All functions between sets

---

### Function Properties in the Category Set

When we have a function f: A → B (mapping from set A to set B), we can classify it based on three key properties:

| Property | Definition | Visualization | Example |
| ---------- | ----------- | ---------------- | --------- |
| **Injective** (One-to-One) | Every element in A maps to a **different** element in B. No two elements share the same image. | No "collisions" | f(x) = 2x (ℝ → ℝ) |
| **Surjective** (Onto) | Every element in B is mapped **from** at least one element in A. No "orphans" in B. | Every target is hit | f(x) = ⌊x⌋ (ℝ → ℤ) |
| **Bijective** | Both injective AND surjective. Perfect one-to-one correspondence. | Perfect pairing | f(x) = x + 5 (ℝ → ℝ) |

---

### Visual Examples

#### Example 1: Injective but NOT Surjective

```txt
Set A: {1, 2, 3}          Set B: {a, b, c, d, e}

f(1) → a
f(2) → b
f(3) → c

✓ Injective: Each element in A maps to a different element in B
✗ Not Surjective: Elements d and e in B are not mapped to
```

**Real example:** f(x) = 2x from {1, 2, 3} to {2, 4, 6, 8, 10}

- f(1) = 2, f(2) = 4, f(3) = 6
- 8 and 10 are never reached

---

#### Example 2: Surjective but NOT Injective

```txt
Set A: {1, 2, 3, 4, 5}    Set B: {a, b, c}

f(1) → a
f(2) → a
f(3) → b
f(4) → b
f(5) → c

✗ Not Injective: Both 1 and 2 map to a (collision!)
✓ Surjective: Every element in B is hit
```

**Real example:** f(x) = x mod 3 from {0, 1, 2, 3, 4, 5} to {0, 1, 2}

- f(0) = 0, f(1) = 1, f(2) = 2, f(3) = 0, f(4) = 1, f(5) = 2
- Every value 0, 1, 2 is reached, but with "repeats"

---

#### Example 3: Bijective (Both!)

```txt
Set A: {1, 2, 3}          Set B: {a, b, c}

f(1) → a
f(2) → b
f(3) → c

✓ Injective: Each element in A maps to a different element in B
✓ Surjective: Every element in B is mapped to exactly once
```

**Real example:** f(x) = x from {1, 2, 3} to {1, 2, 3}

- Perfect one-to-one correspondence
- Identity function is always bijective

---

#### Example 4: Neither Injective NOR Surjective

```txt
Set A: {1, 2, 3, 4}       Set B: {a, b, c, d, e, f}

f(1) → a
f(2) → a
f(3) → b
f(4) → b

✗ Not Injective: Collisions (1 and 2 both → a)
✗ Not Surjective: c, d, e, f are never reached
```

**Real example:** f(x) = ⌊x/2⌋ from {1, 2, 3, 4} to {0, 1, 2, 3, 4, 5}

- f(1) = 0, f(2) = 1, f(3) = 1, f(4) = 2
- Collision at 1 (2 and 3 both map to it)
- Many targets (3, 4, 5) are never reached

---

### Recap

**Injectivity (One-to-One):**

- ✅ Allows us to **recover the original** from the image
- ✅ Used in **encoding** (no information loss)
- ✅ Used in **hashing** (but perfect hashing requires injectivity + same-sized domains)
- ❌ Makes **compression** impossible

**Surjectivity (Onto):**

- ✅ Ensures **every target is reachable**
- ✅ Used in **decoding** (all symbols are valid)
- ✅ Used in **exhaustive search** (nothing is missed)
- ❌ Means **some sources must collide** if domain < codomain

**Bijectivity (Both):**

- ✅ **Perfect mapping** (like a permutation)
- ✅ Allows **reversible operations** (invertible functions)
- ✅ Used in **encryption** (bijective functions are reversible)
- ✅ Used in **isomorphisms** (bijective morphisms = structure-preserving permutations)
- ❌ Requires source and target sets to have **same cardinality**

---

### Ex08: Powerset Function

```txt
f: Set → PowerSet

f({1, 2}) → {{}, {1}, {2}, {1,2}}

This is BIJECTIVE:
- Injective: Different input sets → different powersets
- Surjective: Every subset appears in the powerset
```

### Ex09: Set Evaluation Function

```txt
f: Formula → Set

f("AB&") → {elements in A ∩ B}

This is SURJECTIVE but NOT always INJECTIVE:
- Surjective: All sets can be expressed as formula results
- Not Injective: Different formulas can give the same result
  (e.g., "A" and "AA&" both evaluate to A)
```

### Morphisms and Function Properties

**Isomorphism = Bijective Morphism**

An isomorphism is a morphism (structure-preserving) that is also bijective!

```txt
f: G → H is an isomorphism if:
  1. f(a ∘ b) = f(a) ★ f(b)  [morphism]
  2. f is bijective              [one-to-one and onto]
```

**Examples**

- **Bijective Morphism:** The logarithm log: (ℝ⁺, ×) → (ℝ, +)
- **Non-bijective Morphism:** Modulo m: (ℤ, +) → (ℤ/mℤ, +) [surjective but not injective]

---

### The Category Perspective

In category theory, we classify morphisms based on their properties:

| Morphism Type | Injective | Surjective | Bijective | Category |
| --------------- | ----------- | ----------- | ----------- | ---------- |
| **Monomorphism** | ✓ | - | - | "injective-like" |
| **Epimorphism** | - | ✓ | - | "surjective-like" |
| **Isomorphism** | ✓ | ✓ | ✓ | "structure-preserving bijection" |
| **Endomorphism** | - | - | - | "maps to itself" |
| **Automorphism** | ✓ | ✓ | ✓ | "bijective self-map" |

By choosing different morphisms, we can define different categories on the same collection of objects

---

### Quick Test: Classify These Functions

```txt
1. f(x) = x² from ℝ to ℝ
   Injective? NO (f(-2) = f(2) = 4)
   Surjective? NO (negative numbers never appear)
   Result: NEITHER

2. f(x) = x + 1 from ℤ to ℤ
   Injective? YES (different inputs → different outputs)
   Surjective? YES (every integer is someone's image)
   Result: BIJECTIVE

3. f(x) = ⌈x⌉ from ℝ to ℤ (ceiling function)
   Injective? NO (f(2.5) = f(2.1) = 3)
   Surjective? YES (every integer is reached)
   Result: SURJECTIVE only
```

---

In our Rust code and algorithms:

- **Hashing functions:** Need to be fast, don't need to be injective (collisions ok)
- **Encoding/Compression:** Injective (can decompress without loss)
- **Encryption:** Bijective (must be reversible)
- **Transformations:** Often morphisms that preserve the domain structure

The `eval_set` function is a **surjective morphism**—multiple formulas can describe the same set.

## Inverse Morphisms and Functional Inverses

### What is an **Inverse Morphism**?

If a morphism f is **bijective**, there exists an **inverse morphism** f⁻¹ such that:

```txt
(f⁻¹ ∘ f)(x) = (f ∘ f⁻¹)(x) = idX(x) = x
```

Where:

- x ∈ X (x is an element in the domain)
- idX is the **identity morphism** over set X
- ∘ denotes **function composition**

Composing a function with its inverse (in either order) gives us back the original element unchanged.

---

### Understanding the Notation

**Function Composition (∘):**

```txt
(f ∘ g)(x) = f(g(x))

"First apply g, then apply f"
```

**Identity Morphism (idX):**

```txt
idX(x) = x

"Do nothing—return the element unchanged"
```

**Inverse Morphism (f⁻¹):**

```txt
f⁻¹(f(x)) = x
f(f⁻¹(y)) = y

"Undo the operation of f"
```

---

### Example 1: Simple Addition

```txt
f(x) = x + 5 from ℝ → ℝ

Inverse: f⁻¹(x) = x - 5

Verification:
(f⁻¹ ∘ f)(x) = f⁻¹(f(x)) = f⁻¹(x + 5) = (x + 5) - 5 = x ✓
(f ∘ f⁻¹)(x) = f(f⁻¹(x)) = f(x - 5) = (x - 5) + 5 = x ✓

Both compositions give the identity
```

---

### Example 2: Logarithm and Exponential

```txt
f(x) = log(x) from ℝ⁺ → ℝ

Inverse: f⁻¹(x) = eˣ

Verification:
(f⁻¹ ∘ f)(x) = f⁻¹(log(x)) = e^(log(x)) = x ✓
(f ∘ f⁻¹)(x) = f(eˣ) = log(eˣ) = x ✓

They perfectly undo each other
```

---

### Example 3: Bit Flip (Boolean)

```txt
f(x) = NOT x from {0, 1} → {0, 1}

Inverse: f⁻¹(x) = NOT x (self-inverse!)

Verification:
(f⁻¹ ∘ f)(0) = NOT(NOT(0)) = NOT(1) = 0 ✓
(f ∘ f⁻¹)(1) = NOT(NOT(1)) = NOT(0) = 1 ✓

NOT is its own inverse
```

---

### The Group Structure of Functions

This is the key insight: **The set of all bijective functions with composition (∘) forms a group!**

Let's verify the group axioms for (Bijections, ∘):

| Axiom | Verification | Example |
| ------- | -------------- | --------- |
| **Closure** | Composing two bijections gives a bijection | f(x) = 2x, g(x) = x+3, then (f∘g)(x) = 2x+6 ✓ |
| **Associativity** | (f∘g)∘h = f∘(g∘h) | Function composition is always associative ✓ |
| **Identity** | idX is the identity element | f ∘ id = f, id ∘ f = f ✓ |
| **Inverse** | Every bijection f has an inverse f⁻¹ | If f is bijective, f⁻¹ exists ✓ |

**Therefore:** (Bijections, ∘) is an **Abelian Group** when restricted to the same domain/codomain!

---

### Why Bijections are Special

| Property | Bijection | Non-bijection |
| ---------- | ----------- | --------------- |
| **Invertible** | ✅ Has a unique inverse | ❌ No inverse exists |
| **Information Loss** | ✅ Reversible (no loss) | ❌ Some info is lost |
| **Group Element** | ✅ Part of a group structure | ❌ Not a group element |
| **Isomorphism** | ✅ Can be an isomorphism | ❌ Cannot be an isomorphism |

---

### Practical Examples: Encryption

Encryption relies entirely on bijections and their inverses!

**AES (Advanced Encryption Standard):**

```txt
Plaintext ──[f: encryption]──> Ciphertext
                ↑
Ciphertext ──[f⁻¹: decryption]──> Plaintext

Requirement: f must be BIJECTIVE!
- If not injective: Two different plaintexts encrypt to same ciphertext (disaster!)
- If not surjective: Some ciphertexts can never be produced (inefficient)
- Must be bijective: Perfect 1-to-1 correspondence
```

---

### Symmetric Property and Reversibility

Notice the **symmetry** in the definition:

```txt
(f⁻¹ ∘ f)(x) = x    [right inverse]
(f ∘ f⁻¹)(x) = x    [left inverse]
```

This symmetry reflects the **group property**: both orders give the identity.

**For bijections specifically:**

- Left inverse = Right inverse = Unique inverse
- This is NOT true for non-bijective functions!

---

### Example: Why Non-bijections Can't Have Inverses

**Function that is NOT injective:**

```txt
f(x) = x² from ℝ → ℝ⁺

Problem: f(-2) = f(2) = 4
         ↓
What should f⁻¹(4) equal? -2 or 2?
There's no consistent answer
```

**Function that is NOT surjective:**

```txt
f(x) = x² from ℝ → ℝ

Problem: -1 is in the codomain but never reached
         ↓
What should f⁻¹(-1) equal?
There's no element to map back to...
```

**Only bijections escape these problems.**

---

## Comprehensive Morphism Taxonomy

Beyond the basic distinction between homomorphisms, isomorphisms, and others, there's a rich vocabulary for describing different types of morphisms:

### The Morphism Spectrum

```txt
All Functions
    ↓
Morphisms (structure-preserving functions)
    ├─ Homomorphisms
    │  ├─ Epimorphisms (surjective)
    │  ├─ Monomorphisms (injective)
    │  └─ Isomorphisms (bijective)
    │
    ├─ Endomorphisms (f: G → G)
    │  ├─ Automorphisms (bijective endomorphisms)
    │  └─ Regular Endomorphisms
    │
    └─ Exomorphisms (f: G → H where G ≠ H)
```

---

### **Homomorphism** (General Structure-Preserving)

**Definition:** A function f: A → B where f(a ∘ b) = f(a) ★ f(b)

**What it preserves:** The algebraic operation

**Examples:**

- f(x) = x mod 5 from (ℤ, +) to (ℤ/5ℤ, +)
- f(x) = det(x) from (GL₂(ℝ), ×) to (ℝ*, ×)
- f(x) = log(x) from (ℝ⁺, ×) to (ℝ, +)

**Properties:**

- ✅ Must preserve the operation
- ❌ Don't have to be injective
- ❌ Don't have to be surjective
- ❌ Don't have to be reversible

**Real-world use:** Simplifying complex structures

---

### **Monomorphism** (Injective Morphism)

**Definition:** A morphism f: A → B that is **injective** (one-to-one)

**Intuition:** "No information loss" — we can always recover the original

**Examples:**

- f(x) = 2x from (ℝ, +) to (ℝ, +)
- f(x) = x from any group to itself (inclusion)
- f(x) = log(x) from (ℝ⁺, ×) to (ℝ, +)

**Properties:**

- ✅ Structure-preserving (morphism)
- ✅ No collisions (injective)
- ❌ May not hit every target (not necessarily surjective)
- ✅ Left-cancellative: if f ∘ g₁ = f ∘ g₂, then g₁ = g₂

**Real-world use:** Embedding one structure inside another

---

### **Epimorphism** (Surjective Morphism)

**Definition:** A morphism f: A → B that is **surjective** (onto)

**Intuition:** "Covers everything" — every element in the target is used

**Examples:**

- f(x) = x mod 5 from (ℤ, +) to (ℤ/5ℤ, +)
- f(x) = ⌊x⌋ from (ℝ, +) to (ℤ, +)
- f(x) = x from any group to its quotient group

**Properties:**

- ✅ Structure-preserving (morphism)
- ✅ Covers entire codomain (surjective)
- ❌ May have collisions (not necessarily injective)
- ✅ Right-cancellative: if g₁ ∘ f = g₂ ∘ f, then g₁ = g₂

**Real-world use:** Quotient structures, abstractions

---

### **Isomorphism** (Bijective Morphism)

**Definition:** A morphism f: A → B that is **bijective** (both injective and surjective)

**Intuition:** "Perfect correspondence" — A and B are "the same" algebraically

**Examples:**

- f(x) = x + c from (ℝ, +) to (ℝ, +)
- f(x) = log(x) from (ℝ⁺, ×) to (ℝ, +)
- f(x) = 2πr from circles to their circumferences

**Properties:**

- ✅ Structure-preserving (morphism)
- ✅ One-to-one correspondence (bijective)
- ✅ Has a unique inverse f⁻¹
- ✅ If A and B are isomorphic, they're "essentially identical"

**Real-world use:** Proving two structures are fundamentally the same

---

### **Endomorphism** (Self-Mapping Morphism)

**Definition:** A morphism f: G → G (maps a structure to itself)

**Intuition:** "Internal transformation" — staying within the same structure

**Examples:**

- f(x) = -x in (ℤ, +)
- f(A) = NOT A in ({0, 1}, AND)
- f(x) = x² in (ℝ, ×)
- f(x) = x mod 5 in (ℤ, +)

**Properties:**

- ✅ Structure-preserving
- ✅ Stays within the same set
- ❌ May or may not be bijective
- ❌ May or may not have an inverse

**Subtypes:**

- **Surjective Endomorphism:** Covers all elements
- **Injective Endomorphism:** No collisions
- **Idempotent Endomorphism:** f ∘ f = f (applying twice does nothing more)

**Real-world use:** Symmetries, internal transformations, stabilization

---

### **Automorphism** (Bijective Endomorphism)

**Definition:** An endomorphism f: G → G that is **bijective**

**Intuition:** "Reversible self-transformation" — a symmetry of the structure

**Examples:**

- f(x) = -x in (ℝ, +) [reflection through origin]
- f(x) = NOT x in ({0, 1}, XOR) [bit flip]
- f(A) = A⁻¹ in a group G [inversion]
- Rotations of a square {0°, 90°, 180°, 270°}

**Properties:**

- ✅ Structure-preserving
- ✅ Bijective (has inverse)
- ✅ Invertible: f⁻¹ is also an automorphism
- ✅ Compose easily: f ∘ g is an automorphism if both are
- ✅ Form a group under composition: **Aut(G)**

**The Automorphism Group:**
The set of all automorphisms of a structure G with composition forms a group!

```txt
Example: Automorphisms of ({0, 1}, XOR)
- f₁(x) = x (identity)
- f₂(x) = NOT x (bit flip)

f₁ ∘ f₂ = NOT (composition of automorphisms)
f₂ ∘ f₂ = identity
(Aut({0, 1}, XOR), ∘) is a group!
```

**Real-world use:**

- Symmetries in physics
- Group theory
- Cryptography (permutations are automorphisms)

---

### **Exomorphism** (Morphism Between Different Structures)

**Definition:** A morphism f: G → H where G ≠ H

**Intuition:** "Translation between structures" — moving from one structure to another

**Subtypes:**

- **Exomorphic Homomorphism:** General translation, may lose or add information
- **Exomorphic Monomorphism:** Injective translation (one-to-one)
- **Exomorphic Epimorphism:** Surjective translation (onto)
- **Exomorphic Isomorphism:** Bijective translation (perfect correspondence)

**Examples:**

- f(x) = log(x) from (ℝ⁺, ×) to (ℝ, +) [isomorphic exomorphism]
- f(x) = x mod 5 from (ℤ, +) to (ℤ/5ℤ, +) [epimorphic exomorphism]
- f(x) = 2x from (ℝ, +) to (ℝ, +) with different interpretations [monomorphic exomorphism]

**Real-world use:** Encoding between different representations, translating problems

---

### Complete Morphism Classification Table

```txt
Type           | Domain    | Codomain  | Injective | Surjective | Bijective | Invertible
═══════════════════════════════════════════════════════════════════════════════════════
Homomorphism   | Any       | Any       |     ?     |     ?      |     ?     |    ✗
Monomorphism   | Any       | Any       |     ✓     |     ?      |     ?     |    ✗
Epimorphism    | Any       | Any       |     ?     |     ✓      |     ?     |    ✗
Isomorphism    | Any       | Any       |     ✓     |     ✓      |     ✓     |    ✓
Endomorphism   | G         | G         |     ?     |     ?      |     ?     |    ?
Automorphism   | G         | G         |     ✓     |     ✓      |     ✓     |    ✓
Exomorphism    | G         | H (≠G)    |     ?     |     ?      |     ?     |    ?
```

---

#### Ex00-Ex03: Boolean Operations

```txt
f(x) = NOT x from ({0, 1}, XOR) to ({0, 1}, XOR)

This is an AUTOMORPHISM:
- Endomorphism: Maps to itself
- Bijective: 0 ↔ 1 (one-to-one, onto)
- Invertible: f⁻¹ = f (self-inverse)
- f ∘ f = identity
```

#### Ex05-Ex06: Formula Transformations (NNF, CNF)

```txt
f: Formula → NNF Formula

This is an ENDOMORPHISM:
- Maps formulas to formulas
- Injective? NO (multiple paths to NNF)
- Surjective? NO (not all formulas are in NNF)
- Invertible? NO (information may be hidden)
- Result: Regular endomorphism, not auto
```

#### Ex08: Powerset Bijection

```txt
f: Set → PowerSet(Set)

This is an ISOMORPHIC EXOMORPHISM:
- Exomorphism: Maps sets to their powersets
- Injective: Different sets → different powersets
- Surjective: Every subset appears
- Bijective: Perfect correspondence
- Can be inverted in principle
```

#### Ex09: Set Evaluation

```txt
f: Formula → Set

This is a SURJECTIVE EXOMORPHISM:
- Exomorphism: Maps formulas to sets
- Homomorphism: f(A & B) = f(A) ∩ f(B)
- Surjective: Any set can be expressed as formula result
- Injective? NO: "A" and "A|A" both evaluate to A
- Invertible: NO (not bijective)
```

---

### understanding morphism types matters

1. **Classify problems:** Know if transformation is reversible
2. **Design algorithms:** Choose appropriate mapping types
3. **Prove properties:** Use morphism cancellation laws
4. **Understand symmetry:** Automorphisms encode structure symmetries
5. **Build abstractions:** Endomorphisms help organize computations

The vocabulary precision allows mathematicians and computer scientists to talk about transformations with incredible specificity! 

---

### Quick Reference: When to Use Each

| When We Need | Use | Why |
| --------------- | --------------- | ----- |
| General structure preservation | Homomorphism | Simplest requirement |
| No information loss | Monomorphism | Can recover original |
| Coverage guarantee | Epimorphism | All targets reachable |
| Perfect translation | Isomorphism | Structures are identical |
| Internal symmetry | Automorphism | Self-transformation |
| Reversible operation | Bijective morphism | Can undo it |
| Embedding one in another | Monomorphic exomorphism | Injection into larger space |
| Quotient/abstraction | Epimorphic endomorphism | Grouping equivalent elements |

## Space-Filling Curves

### What is a **Space-Filling Curve**?

A **space-filling curve** is a continuous curve that maps a closed interval [0, 1] ∈ ℝ to a set of values in one or more dimensions, covering the entire space.

**Mathematical Definition:**

```txt
f : [0, 1] → Mⁿ
```

Where:

- **[0, 1]** = the source set (a 1D interval containing values like 0.543543, 0.3333..., etc.)
- **Mⁿ** = an n-dimensional manifold (geometric shape or space)
- **n ∈ ℕ** = the number of dimensions we wish to cover
- **f must be bijective** = every point in [0, 1] maps to a unique point in Mⁿ, and every point in Mⁿ is reached

**Intuition:** Imagine "unrolling" a 2D space (like a square) into a 1D line in such a way that:

1. Every point on the line maps to exactly one point in the square (injective)
2. Every point in the square is reached from exactly one point on the line (surjective)
3. The mapping is continuous (no jumps or breaks)

---

### What is meant by "Space-Filling"?

The term **space-filling** is slightly misleading in classical mathematics:

- In the strict topological sense, a truly continuous space-filling curve **cannot** be bijective
- However, we can get arbitrarily close with discrete approximations
- For practical computational purposes, we treat them as bijective mappings

In the **Ex10 (Curve Saturation)**, we're working with a discrete approximation that IS truly bijective.

---

### Classical Examples

#### Example 1: Hilbert Curve

The **Hilbert Curve** is perhaps the most famous space-filling curve.

**Properties:**

- Maps [0, 1] to a 2D square [0, 1]²
- Recursive fractal structure
- Preserves locality (nearby points on the line map to nearby regions in the square)

**Visual Progression (Iterations 1-4):**

```txt
Iteration 1:     Iteration 2:        Iteration 3:           Iteration 4:
┌─────┐         ┌───┬───┐          ┌─┬─────┬─┐            (very detailed)
│  →  │         │ ┌─┐ ┌─┐│          │ ├──┬──┤ │             Each generation
└─────┘         └─┘ └─┘ └┘          └─┴──┴──┴─┘             subdivides more
```

**Key property:** Each iteration gets closer to filling the entire square while maintaining continuity.

---

#### Example 2: Peano Curve

The **Peano Curve** was the first discovered space-filling curve (1890).

**Properties:**

- Maps [0, 1] to a 2D square [0, 1]²
- More chaotic than Hilbert (doesn't preserve locality as well)
- Purely recursive definition

---

#### Example 3: Z-Order Curve (Morton Code)

The **Z-Order Curve** (also called Morton curve or Morton code) is simpler and more computational-friendly.

**Properties:**

- Maps [0, 1] to an n-dimensional hypercube [0, 1]ⁿ
- Easily computed using bit-interleaving
- Used extensively in computer graphics and databases

**How it works:**

```txt
For 2D: Interleave bits of x and y coordinates

x = 0b101 = 5
y = 0b011 = 3

Z-order = 0b 01 00 11 01 = 0011 0101 (binary interleaving)

The resulting curve follows a Z-pattern at each scale!
```

---

### Ex10: Discrete Space-Filling Curve

In **Ex10 (Curve Saturation)**, we implement a bijective mapping between:

```txt
Source: Discrete 2D grid [0, width) × [0, height)
Target: 1D linear array [0, width * height)
```

**Our function signature:**

```rust
fn map(x: u16, y: u16) -> u32 {
    // Maps (x, y) coordinate to a 1D index
}
```

**Key characteristics:**

- ✅ **Truly bijective:** Every (x, y) maps to unique index, every index reached once
- ✅ **Discrete:** Works on integer coordinates (not continuous reals)
- ✅ **Efficient:** Single formula, no loops
- ✅ **Space-filling:** Covers all 2^32 points in the grid without gaps

**Why it's a discrete space-filling curve:**

```txt
Input space: u16 × u16 = 2^16 × 2^16 = 2^32 total points
Output space: u32 = 2^32 total values

The function creates a bijection between these spaces.
```

---

### Connection to Morphism Theory

The **Ex10 map function is a bijective morphism (isomorphism)!**

```txt
f: (ℤ/2¹⁶ℤ × ℤ/2¹⁶ℤ, +) → (ℤ/2³²ℤ, +)

Properties:
- Homomorphic: Preserves structure (grid ordering becomes linear ordering)
- Bijective: One-to-one correspondence between grid points and indices
- Invertible: There's an inverse function f⁻¹ that maps indices back to (x, y)
- Automorphism-like: Both domain and codomain have the same cardinality (2³²)
```

---

### Properties of Space-Filling Curves

| Property | Meaning | Ex10 |
| ---------- | --------- | ------ |
| **Continuity** | No jumps in the curve |  If implemented carefully |
| **Bijection** | One-to-one correspondence |  Guaranteed by design |
| **Locality** | Nearby points map to nearby regions | Depends on implementation |
| **Computational** | Efficiently computable |  Single formula |
| **Invertible** | Can reverse the mapping |  Inverse function possible |

---

### Types of Space-Filling Curves

#### 1. **Hilbert Curve**

- ✅ Best locality preservation
- ✅ Most "natural" ordering
- ❌ More complex to compute
- **Use when:** Spatial locality matters (caching, clustering)

#### 2. **Peano Curve**

- ✅ First discovered, historical importance
- ❌ Worse locality than Hilbert
- ❌ More complex computation
- **Use when:** Pure mathematical interest

#### 3. **Z-Order Curve (Morton Code)**

- ✅ Easy to compute (bit interleaving)
- ✅ Extends to n dimensions easily
- ❌ Locality not as good as Hilbert
- **Use when:** Need simplicity and efficiency (Ex10)

#### 4. **Gray Code Curve**

- ✅ Single-bit changes between adjacent elements
- ✅ Useful for error correction
- ❌ More complex than Z-order
- **Use when:** Working with error-prone systems

---

### Z-Order Curve: The Ex10 Implementation

The **Z-Order (Morton Code)** curve for Ex10:

**Bit Interleaving Formula:**

```txt
For 2D coordinates (x, y) to index i:

Take each bit of x and y alternately:
x = ...x₃ x₂ x₁ x₀
y = ...y₃ y₂ y₁ y₀

Result = ...y₃ x₃ y₂ x₂ y₁ x₁ y₀ x₀

This creates a zigzag pattern that covers the entire grid!
```

**Example with small numbers:**

```txt
Point (1, 2):
x = 1 = 0b01
y = 2 = 0b10

Interleaved: 0b 1 0 0 1 = 9
            (y₁ x₁ y₀ x₀)

The pattern: (0,0)→(1,0)→(0,1)→(1,1)→(2,0)... follows a Z shape
```

1. ✅ Both x and y are u16, total 2³² combinations
2. ✅ Output index fits in u32 (2³² values)
3. ✅ Perfect bijection with bit interleaving
4. ✅ No gaps, no collisions
5. ✅ Continuous in the discrete sense

---

### Real-World Applications of Space-Filling Curves

#### 1. **Database Indexing**

```txt
Multi-dimensional spatial data → 1D index
Used in: Geographic databases, image retrieval systems
Example: PostGIS uses space-filling curves for spatial indexing
```

#### 2. **Image Processing**

```txt
2D image pixels → 1D array following curve pattern
Benefit: Improves cache locality, better compression
```

#### 3. **Graphics & Gaming**

```txt
3D coordinates → 1D texture coordinates
Used in: Volumetric rendering, procedural generation
```

#### 4. **Data Compression**

```txt
Multi-dimensional data → 1D sequence
Benefit: Nearby data points are close in sequence
Result: Better compression ratios
```

#### 5. **Memory Organization**

```txt
2D grid access → Linear memory layout
Benefit: Improves CPU cache performance
```

#### 6. **Cryptography**

```txt
Confusion/diffusion properties
Space-filling curves shuffle data across dimensions
```

---

### Mathematical Deep Dive: Why Bijection Matters

**For a space-filling curve to work:**

```txt
Must be SURJECTIVE:
- Every point in the target space (grid) must be reached
- No "holes" or uncovered regions
- Example: If using u32 for 2D u16×u16, all 2³² values used

Must be INJECTIVE:
- No two source points map to the same target
- No collisions or overlaps
- Example: Two different (x,y) pairs never give same index

Must be BIJECTIVE:
- Both injective AND surjective
- Perfect one-to-one correspondence
- Invertible: can go from index back to (x,y)
```

**This is why Ex10 requires a bijective function—without it, we'd have gaps or collisions in our curve**

---

### Computing Space-Filling Curves

#### Hilbert Curve (Complex)

```rust
fn hilbert_index(x: u16, y: u16, order: u32) -> u32 {
    let mut index = 0;
    let mut s = order / 2;
    
    while s > 0 {
        let rx = ((x & s) > 0) as u32;
        let ry = ((y & s) > 0) as u32;
        index += s as u32 * s as u32 * ((3 * rx) ^ ry);
        
        // Rotate coordinates
        if ry == 0 {
            if rx == 1 {
                // Rotate 90 degrees...
            }
        }
        s /= 2;
    }
    index
}
```

#### Z-Order Curve (Simple!)

```rust
fn z_order_index(mut x: u16, mut y: u16) -> u32 {
    let mut index = 0;
    
    for i in 0..16 {
        index |= ((x & (1 << i)) as u32) << i;
        index |= ((y & (1 << i)) as u32) << (i + 1);
    }
    index
}
```

The Z-Order is much simpler and perfect for Ex10!

---

### Connection to All The Concepts

```txt
Morphism Theory:
  Our map function is an isomorphism between
  (u16 × u16, ordering) and (u32, linear ordering)

Group Theory:
  The discrete grid forms a group under addition
  Our mapping preserves this group structure

Sets & Functions:
  Domain: Set of all (u16, u16) pairs
  Codomain: Set of all u32 values
  Bijection required: |Domain| = |Codomain| = 2³²

Categories:
  This is an object in the category Set
  Our function is a morphism (specifically an isomorphism)
```

---

### Why This Matters for Saturation

In **Curve Saturation**, the space-filling curve ordering becomes important:

```txt
Without proper mapping:
  Random access to (x, y) → unpredictable memory patterns
  Poor cache performance, inconsistent access times

With space-filling curve:
  (x, y) ordered along curve → predictable memory patterns
  Better cache locality, consistent performance
  Easier to parallelize and vectorize
```

The bijective property ensures:

- ✅ Every pixel visited exactly once
- ✅ No gaps to fill
- ✅ No redundant work
- ✅ Optimal saturation coverage

---

### Mathematical Beauty

Space-filling curves demonstrate several deep mathematical principles:

1. **Continuity vs. Discreteness:** Classical curves are continuous, but discrete approximations work perfectly in practice

2. **Dimensionality Reduction:** Mapping n-D spaces to 1-D while preserving structure

3. **Bijection Power:** Perfect one-to-one correspondence enables lossless transformations

4. **Fractal Self-Similarity:** Many space-filling curves exhibit recursive, fractal patterns

5. **Morphism Theory in Action:** The Ex10 is a concrete example of isomorphic structure-preserving mapping

Space-filling curves are where **pure mathematics meets practical computation**! 🌟

## Understanding Continuity in Space-Filling Curves

### What is **Continuity** in General?

**Mathematical Definition:** A function f: A → B is **continuous** if small changes in the input produce small changes in the output. More formally:

```txt
For every ε > 0, there exists a δ > 0 such that:
if |x - y| < δ, then |f(x) - f(y)| < ε

"Arbitrarily small input changes lead to arbitrarily small output changes"
```

**Intuition:** We can draw the function without lifting the pen from the paper. No jumps, breaks, or sudden discontinuities.

---

### Continuity in Space-Filling Curves

In the context of space-filling curves, **continuity means:**

```txt
f : [0, 1] → M²  (or M^n in higher dimensions)

If we have two points t₁ and t₂ that are CLOSE on the line [0, 1],
then their images f(t₁) and f(t₂) must be CLOSE in the 2D space M².
```

**Visual Example (2D Square):**

```txt
Line [0, 1]:
0.000 ─── 0.333 ─── 0.666 ─── 1.000
 │         │         │         │
 ↓         ↓         ↓         ↓
(0,0) ──→ (0,1) ──→ (1,1) ──→ (1,0)

Points close on the line map to points close in the square!
The curve doesn't "teleport" - it travels continuously.
```

---

### The Classical Paradox: Peano's Discovery (1890)

This is where space-filling curves get fascinating and counterintuitive!

**Peano proved:** A **continuous** function CAN map [0, 1] bijectively (one-to-one) onto a **2D square** [0, 1]²!

This shocked mathematicians because:

1. ✅ The curve IS continuous (no jumps)
2. ✅ The curve IS surjective (covers every point)
3. ✅ The curve IS injective (no two parameters map to same point)
4. ❌ YET it appears to "fold" space in impossible ways!

**How is this possible?** The answer: **The curve must be nowhere differentiable** (infinitely "wiggly")

---

### Three Levels of Understanding Continuity

#### Level 1: Intuitive

```txt
Continuous: Can draw without lifting pen
Discontinuous: Must lift pen (has jumps)

Space-filling curve: A pen-drawn path that somehow touches every point
in a 2D square, yet never crosses itself!
```

#### Level 2: Geometric (What We See)

For a curve f: [0, 1] → ℝ²:

```txt
CONTINUOUS:
  Point at t=0.5:     f(0.5) = (0.5, 0.5)
  Point at t=0.5001:  f(0.5001) ≈ (0.5001, 0.5001)  [very close!]
  
  The curve "travels" smoothly through space.

DISCONTINUOUS:
  Point at t=0.5:     f(0.5) = (0.2, 0.8)
  Point at t=0.5001:  f(0.5001) = (0.9, 0.1)  [far away!]
  
  The curve "teleports" - big jump!
```

#### Level 3: Formal (What Math Says)

**Topological Definition:**

A function f: X → Y is continuous if:

- For every open set U in Y, f⁻¹(U) is open in X
- (Pre-images of open sets are open)

**Metric Definition:**

A function f: X → Y is continuous at point x₀ if:

```txt
For all ε > 0, there exists δ > 0 such that:
  distance(x, x₀) < δ  ⟹  distance(f(x), f(x₀)) < ε
```

---

### Examples of Continuous vs Discontinuous Functions

#### Example 1: Simple Continuous Function

```txt
f(t) = (t, t) from [0, 1] to the line y = x in ℝ²

Continuity check:
- t = 0.5:     f(0.5) = (0.5, 0.5)
- t = 0.50001: f(0.50001) = (0.50001, 0.50001)
- Change is tiny in both x and y coordinates ✓

Result: CONTINUOUS
The curve is just a straight diagonal line from (0,0) to (1,1).
```

#### Example 2: Discontinuous Function

```txt
f(t) = { (0, 0)  if t < 0.5
       { (1, 1)  if t ≥ 0.5

Continuity check at t = 0.5:
- t = 0.4999: f(0.4999) = (0, 0)
- t = 0.5000: f(0.5000) = (1, 1)
- Distance = √((1-0)² + (1-0)²) = √2 ≈ 1.414 (HUGE jump!)

Result: DISCONTINUOUS
The curve teleports from one corner to the opposite corner!
```

#### Example 3: Hilbert Curve

```txt
f: [0, 1] → [0, 1]²

Properties:
✓ Continuous: Small changes in t lead to small changes in (x, y)
✓ Bijective: Every point in the square is hit exactly once
✓ Nowhere differentiable: Infinitely wiggly (no smooth parts)
✓ Self-avoiding: Never crosses itself

Paradox: How can it be continuous AND bijective AND nowhere differentiable?
Answer: Because continuity doesn't require smoothness—just no jumps!
```

---

### Why Continuity Matters for Space-Filling Curves

**Without Continuity:**

```txt
Parameter t = 0.333333
Curve position = (0.1, 0.9)

Parameter t = 0.333334  [infinitesimal change!]
Curve position = (0.8, 0.2)  [completely different region!]

Result: Curve "jumps around" the space randomly
No coherent path, no nice properties
```

**With Continuity:**

```txt
Parameter t = 0.333333
Curve position = (0.1, 0.9)

Parameter t = 0.333334  [infinitesimal change]
Curve position = (0.100001, 0.900001)  [very close!]

Result: Curve travels smoothly, touching nearby points first
Before reaching distant regions
```

---

### The Continuity-Bijectivity Paradox Explained

**Classical Theorem (Surprising but True):**

```txt
A continuous bijection from [0, 1] to [0, 1]² EXISTS!

But it cannot be SMOOTH (differentiable everywhere).
It must be nowhere differentiable (infinitely jagged).
```

**Why?**

Think about a smooth curve in 2D:

- It's 1-dimensional (thin line)
- A 2D square has measure (area)
- A smooth 1D line CANNOT fill a 2D area

But if the curve is **infinitely wiggly** (fractal-like):

- It gets dense enough to touch every point
- Yet remains continuous (no jumps)
- It's a 1D curve in 2D space with Hausdorff dimension 2!

**Analogy:**

```
Smooth curve: A straight highway (misses most of the territory)
Fractal curve: A road network with infinite detail (visits everywhere)
```

---

### Ex10: Discrete Continuity

In **Ex10 (Curve Saturation)**, the discrete space-filling curve should preserve **discrete continuity**:

```rust
fn map(x: u16, y: u16) -> u32 {
    // Bijective mapping from 2D grid to 1D line
}
```

**Discrete Continuity Check:**

```txt
Point (x, y):         map(x, y) = index₁
Point (x+1, y):       map(x+1, y) = index₂
Point (x, y+1):       map(x, y+1) = index₃

For DISCRETE CONTINUITY:
|index₂ - index₁| should be SMALL
|index₃ - index₁| should be SMALL

"Neighboring grid points map to nearby indices"
```

**Why It Matters for Saturation:**

```txt
If we process pixels in curve order:
pixel[0] → nearby pixels [1, 2, 3]
pixel[1] → nearby pixels [0, 2, 3]

This preserves spatial locality.
Better cache performance, faster processing.
```

---

### Comparing Continuous vs Discrete Space-Filling Curves

| Property | Continuous (Classical) | Discrete (Ex10) |
| ---------- | ------------------------ | ---------------------- |
| **Domain** | [0, 1] (real numbers) | {0, 1, ..., 2³²-1} |
| **Codomain** | [0, 1]² (real plane) | u16 × u16 grid |
| **Bijective** | YES | YES |
| **Continuous** | YES | YES (in discrete sense) |
| **Differentiable** | NO (nowhere) | N/A (discrete) |
| **Computable** | Hard (infinite detail) | Easy (formula) |
| **Implementation** | Fractal recursive | Bit interleaving |

---

### Formal Definition: Continuous Space-Filling Curve

**Mathematical Definition:**

```txt
A continuous space-filling curve is a function f: [0, 1] → ℝⁿ such that:

1. f is CONTINUOUS:
   ∀ε > 0, ∃δ > 0: |t - s| < δ ⟹ ||f(t) - f(s)|| < ε

2. f is SURJECTIVE (onto):
   For every point p in the target space, ∃t ∈ [0,1]: f(t) = p

3. f is INJECTIVE (one-to-one):
   If t ≠ s, then f(t) ≠ f(s)

4. f is BIJECTIVE:
   Combination of 2 and 3: perfect one-to-one correspondence
```

**The Paradox:**

- Property 1 says it's a "thin" curve (continuous)
- Property 2 says it covers an entire "thick" region (surjective)
- Yet property 3 says it never overlaps (injective)

**Resolution:** The curve must be **nowhere differentiable** (infinitely complex)
