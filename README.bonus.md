# Group Theory

## Set vs Group: The Fundamental Difference

### What is a Set?
A set is just a collection of distinct elements without any underlying structure.

**Examples:**
* {1, 2, 3} - three numbers
* {apple, banana, orange} - three fruits
* {A, B, C} - three variables

**Properties of sets:**
* Elements are distinct, meaning there are no duplicates.
* Order does not matter, so {1, 2, 3} is the same as {3, 2, 1}.
* There are no operations defined.
* There are no relationships between elements.

Set operations like union (A ∪ B), intersection (A ∩ B), and complement (A') add actions on sets, but they do not create a group structure.

---

### What is a Group?
A group is a set paired with a binary operation that satisfies four special properties. 

A group (G, ∘) consists of a set G and a binary operation ∘ used to combine two elements.

**The 4 Group Axioms:**

| Property | Meaning | Example (ℤ, +) |
| -------- | ------- | --------------- |
| **Closure** | Combining any two elements gives another element in the group. | 3 + 5 = 8 ∈ ℤ |
| **Associativity** | (a ∘ b) ∘ c = a ∘ (b ∘ c) | (1 + 2) + 3 = 1 + (2 + 3) |
| **Identity Element** | There exists an element e where a ∘ e = a for all a. | 0 + 5 = 5 |
| **Inverse Element** | For each element a, there exists a⁻¹ where a ∘ a⁻¹ = e. | 5 + (-5) = 0 |

---

### Examples: Set vs Group

#### Example 1: Natural Numbers
Natural numbers ℕ = {0, 1, 2, 3, ...} form a set with no operation. When paired with addition to form (ℕ, +), it has closure, associativity, and an identity (0), but it fails the inverse property because there are no negative numbers in ℕ. Therefore, (ℕ, +) is not a group, though (ℤ, +) is.

#### Example 2: Boolean Values
Boolean values {0, 1} paired with the XOR operation form a group. This group has closure (0 XOR 1 = 1), associativity, an identity (0), and each element is its own inverse (1 ⊕ 1 = 0).

#### Example 3: Rotation of a Square
The rotation of a square uses the set of angles {0°, 90°, 180°, 270°}. Paired with rotation, it has closure (90° + 180° = 270°), associativity, an identity (0°), and inverses where 90° and 270° cancel out. This forms the Cyclic Group of Order 4.

---

### Quick Comparison Table

| Concept | Set | Group |
| --------- | ----- | ------- |
| **Definition** | Collection of elements | Set + binary operation |
| **Structure** | No relationships | Elements can be combined |
| **Example** | {apple, banana, cherry} | (ℤ, +) integers with addition |
| **Requirements** | None | 4 axioms must hold |
| **Can be empty?** | Yes | No, must have identity |
| **Operation needed?** | No | Yes, exactly one |

---

### Connection to My Boolean Algebra Project
In my exercises:
* **Sets** (Ex08 Powerset): Collecting all subsets like {}, {A}, {B}, and {A,B}.
* **Operations** (Ex00-Ex06): Defining ∧, ∨, and ¬ on sets.
* **Group structure**: The boolean values {0, 1} with XOR form an abelian group.

Boolean algebra relies heavily on group theory. XOR is a group operation that is particularly useful in cryptography, and understanding groups directly helps with circuit design and abstract algebra.

Groups represent symmetry in mathematics. Rotations of objects, permutations of elements, symmetries of crystals, and molecular chemistry all use group theory. Any time there is a reversible operation like addition, XOR, or rotation, a group is likely present.

## The Order of a Group
The order of a group is simply the number of elements in its underlying set.

### 1. Finite vs. Infinite Orders
* **Finite Groups:** If a group has a specific number of elements, its order is that number. For example, the group of Boolean values has an order of 2, while a 2x2 Rubik's Cube group has an order of 3,674,160.
* **Infinite Groups:** If the set is unbounded like the Integers, the group has an infinite order.

### 2. Cardinality
Cardinality refers to the size of the set, regardless of how elements are arranged or what the operation is. In my project, a truth table for variables has rows, representing a Boolean space order.

---

### 3. Order of an Element
The order of a group is different from the order of an element. The order of an element is the smallest positive integer where applying the operation that many times returns to the identity. 

In the XOR group, the group order is 2, and the order of element 1 is also 2 since 1 ⊕ 1 = 0.

---

### Ex10: Bringing it all together
For Ex10 (Curve Saturation), the work happens within a discrete coordinate space. If variables are u16, they each have 65,536 possible values, creating a grid space with a cardinality of 2^32. The map function takes these discrete points and maps them to points on a 1D line. Because this mapping is symmetrical and bijective, it rearranges the order of the points without losing a single one.

## Morphisms: Structure-Preserving Maps

### What is a Morphism?
A morphism is a function between two algebraic structures that preserves their structure. The term comes from the Greek word "morphe" for form or shape. It acts as a respectful translation between structures rather than an arbitrary map.

---

### Example of a Morphism
For two groups, Group A with operation ∘ and Group B with operation ★, a morphism f from A to B means f(a ∘ b) = f(a) ★ f(b). Combining first and then mapping gives the exact same result as mapping first and then combining.

For example, mapping integers under addition to modulo 5 addition using f(x) = x mod 5 is a morphism. Evaluating f(5 + 3) yields f(8), which is 3. Similarly, f(5) + f(3) is 0 + 3, which also equals 3.

---

### Types of Morphisms

| Type | Name | Definition | Example |
| ------ | ------ | ----------- | --------- |
| **Homomorphism** | Structure-preserving | f(a ∘ b) = f(a) ★ f(b) | f(x) = x mod 5 |
| **Isomorphism** | Bijective + structure-preserving | Homomorphism that's 1-to-1 and onto | f(x) = 2x (ℝ → ℝ) |
| **Endomorphism** | Maps to itself | Morphism f: G → G | f(x) = x² in (ℝ, ×) |
| **Automorphism** | Bijective endomorphism | Isomorphism f: G → G | f(x) = -x in (ℤ, +) |

---

### Practical Examples
A logarithm function f(x) = log(x) mapping positive reals with multiplication to all reals with addition is an isomorphism. It preserves the structure (log(a × b) = log(a) + log(b)) and every positive number has a unique log.

Mapping boolean XOR to integers mod 2 is also an isomorphism. It is bijective and preserves the structure since 1 ⊕ 1 = 0 and 1 + 1 = 0 mod 2.

The determinant function from 2x2 invertible matrices to non-zero real numbers is a homomorphism, but not an isomorphism because it is not bijective.

---

### Real World Applications
Morphisms help classify structures, meaning isomorphic groups are algebraically identical. They simplify problems by mapping a complex structure to a simpler one to solve the problem. In computer science, hashing is a homomorphism, while encoding, decoding, and cryptography heavily rely on morphisms. In physics, physical symmetries are automorphisms.

---

### Connection to My Project
In the context of the project:
* Operations defined on {0, 1} form group structures, and f(x) = NOT x is an automorphism of ({0, 1}, XOR).
* Converting formulas to normal forms (NNF, CNF) is a morphism that preserves the formula structure.
* The powerset operation in Ex08 is a functor mapping sets to sets of subsets while preserving structure.
* The eval_set function in Ex09 is a homomorphism that maps formulas to their set-theoretic meaning.

To check for a morphism, define two structures, pick a function f, and test if f(a ∘ b) = f(a) ★ f(b). An isomorphism is the strongest type because it preserves everything, whereas a regular function preserves nothing.

## Categories and Function Properties

### What is a Category?
A category is a collection of objects and morphisms between them that follow certain rules. The category "Set" includes all sets as objects and all functions between sets as morphisms.

---

### Function Properties
Functions from set A to set B have three key properties:

| Property | Definition | Visualization | Example |
| ---------- | ----------- | ---------------- | --------- |
| **Injective** (One-to-One) | Every element in A maps to a different element in B. | No collisions. | f(x) = 2x (ℝ → ℝ) |
| **Surjective** (Onto) | Every element in B is mapped from at least one element in A. | Every target is hit. | f(x) = ⌊x⌋ (ℝ → ℤ) |
| **Bijective** | Both injective and surjective. | Perfect pairing. | f(x) = x + 5 (ℝ → ℝ) |

---

### Examples in Action
* f(x) = 2x from {1, 2, 3} to {2, 4, 6, 8, 10} is injective but not surjective because 8 and 10 are never reached.
* f(x) = x mod 3 from {0, 1, 2, 3, 4, 5} to {0, 1, 2} is surjective but not injective due to collisions.
* f(x) = x from {1, 2, 3} to {1, 2, 3} is bijective.
* f(x) = ⌊x/2⌋ from {1, 2, 3, 4} to {0, 1, 2, 3, 4, 5} is neither injective nor surjective.

Injectivity allows recovering the original input and is used in encoding, but makes compression impossible. Surjectivity ensures every target is reachable and is used in decoding, but requires collisions if the domain is smaller than the codomain. Bijectivity provides perfect mapping for reversible operations and is used in encryption.

In my project, the Powerset function in Ex08 is bijective because different input sets yield different powersets and every subset appears. The set evaluation function in Ex09 is surjective but not always injective, as different formulas can yield the same result.

In category theory, an isomorphism is a bijective morphism. A monomorphism is injective-like, an epimorphism is surjective-like, an endomorphism maps to itself, and an automorphism is a bijective self-map.

In Rust code, hashing functions are fast and not injective, compression is injective, encryption is bijective, and the eval_set function is a surjective morphism.

## Inverse Morphisms and Functional Inverses

### Inverse Morphisms
If a morphism f is bijective, it has an inverse morphism f⁻¹ where composing the function with its inverse returns the original element unchanged. 

For example, f(x) = x + 5 has an inverse f⁻¹(x) = x - 5. The boolean bit flip f(x) = NOT x is its own inverse.

The set of all bijective functions with composition forms an Abelian Group when restricted to the same domain and codomain. Bijections are invertible, have no information loss, and act as group elements. This is essential for encryption like AES, which strictly requires perfect one-to-one correspondence to function properly. Non-bijections lack inverses because they either have collisions or leave targets unreached.

## Comprehensive Morphism Taxonomy

The morphism spectrum ranges from general homomorphisms to specific automorphisms and exomorphisms.
* **Homomorphism:** Preserves operations and simplifies structures.
* **Monomorphism:** Injective, embedding one structure into another with no information loss.
* **Epimorphism:** Surjective, covering the entire codomain, used for abstractions.
* **Isomorphism:** Bijective, proving two structures are fundamentally identical.
* **Endomorphism:** A self-mapping internal transformation.
* **Automorphism:** A bijective endomorphism representing structural symmetry. The set of automorphisms forms a group under composition.
* **Exomorphism:** Maps between entirely different structures.

In the context of the code:
* Boolean operations like NOT are automorphisms.
* Formula transformations to NNF or CNF are regular endomorphisms, not automorphisms, as they are not bijective.
* The Powerset bijection is an isomorphic exomorphism.
* Set evaluation is a surjective exomorphism.

Understanding morphism types is critical for classifying problems, designing algorithms, proving properties, analyzing symmetry, and building solid code abstractions.

## Space-Filling Curves

### What is a Space-Filling Curve?
A space-filling curve maps a 1D closed interval bijectively to a multi-dimensional space, covering the entire space. In classical topology, a continuous space-filling curve cannot be strictly bijective, but discrete computational approximations are.

**Classical examples:**
* **Hilbert Curve:** Maps to a 2D square with a recursive fractal structure and excellent locality preservation.
* **Peano Curve:** Purely recursive and historically the first discovered.
* **Z-Order Curve (Morton Code):** Maps to an n-dimensional hypercube using bit-interleaving, which is extremely simple to compute.

### Discrete Mapping in Ex10
My Ex10 implements a discrete space-filling curve with a bijective mapping between a 2D grid and a 1D linear array. It works perfectly on integer coordinates, uses a single formula, and covers all points without any gaps. This map function acts as an isomorphism between the grid ordering and the linear ordering.

Space-filling curves are heavily used in database indexing, image processing for cache locality, graphics texture coordinates, data compression, and cryptography. The strict bijective property is necessary to ensure there are no uncovered regions and no overlaps.

### The Z-Order Implementation

The Z-Order curve interleaves bits alternately from the x and y coordinates to create a zigzag pattern that covers the entire grid. This perfectly fits Ex10 since it requires a bijective mapping for cache performance and efficient memory organization. Using this space-filling curve in Curve Saturation guarantees every pixel is visited exactly once with optimal memory layout.

## Understanding Continuity

In general mathematics, a function is continuous if small changes in the input produce small changes in the output. For a space-filling curve, points that are close on the 1D line map to points that are close in the 2D space.

Peano's theorem proved that a continuous bijection onto a 2D square exists, but the curve must be infinitely wiggly and nowhere differentiable. Without continuity, a curve would jump around space randomly, but a continuous curve travels smoothly and preserves spatial structure.

My Ex10 uses a discrete space-filling curve. While not continuous in the strict classical sense, it preserves discrete continuity where neighboring grid points map to reasonably close array indices. This provides solid locality and is much easier to compute than classical fractal curves. Space-filling curves ultimately demonstrate how continuity and bijectivity can comfortably coexist through infinite complexity.