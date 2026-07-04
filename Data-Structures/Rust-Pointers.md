# Rust Pointers — Reference Notes

> Reference for building data structures from scratch (LL, DLL, Tree, Heap, Stack, Graph).
> Each pointer type encodes a different ownership and access pattern in the type system.

## Table of Contents

1. [`&T` / `&mut T` — Borrowed Reference](#1-t--mut-t--borrowed-reference)
2. [`Box<T>` — Single Owner, Heap Allocated](#2-boxt--single-owner-heap-allocated)
3. [`Rc<T>` — Shared Ownership (Single-Threaded)](#3-rct--shared-ownership-single-threaded)
4. [`Weak<T>` — Non-Owning Reference](#4-weakt--non-owning-reference)
5. [`Arc<T>` — Shared Ownership (Thread-Safe)](#5-arct--shared-ownership-thread-safe)
6. [Raw Pointers — `*mut T` / `*const T`](#6-raw-pointers--mut-t--const-t)
7. [Index/Arena Pattern — Honorable Mention](#7-indexarena-pattern-usize-into-a-vec--honorable-mention)
8. [Summary Table](#summary-table)
9. [Quick Decision Path](#quick-decision-path)

---

## 1. `&T` / `&mut T` — Borrowed Reference

### How it works

Not an owning pointer — a temporary borrow with a compile-time-checked **lifetime**.

- `&T` — many readers allowed at once.
- `&mut T` — exactly one writer, no readers at the same time.

Enforced entirely at compile time with zero runtime cost.

### When to use

- Traversing, reading, or mutating a structure that already exists (iterators, accessor methods, helper functions).
- Passing data into functions without transferring ownership.

### When not to use

- Building the structural skeleton itself (node-to-node links) in a recursive or self-referential structure — lifetimes cannot outlive runtime insert/remove operations.
- When a node must be reachable from more than one place, or when the link must live in struct fields long-term rather than being passed around temporarily.

---

## 2. `Box<T>` — Single Owner, Heap Allocated

### How it works

Allocates `T` on the heap with exactly **one owner**. Fixed pointer-sized footprint regardless of `T`, which breaks infinite-size recursion in self-referential types:

```rust
struct Node {
    next: Option<Box<Node>>,
}
```

Dropped automatically when the `Box` goes out of scope — no refcounting, no GC.

### When to use

- Recursive types where size must be known at compile time.
- Structures where ownership flows strictly one way, one owner per node: singly-linked list, simple binary tree (parent owns children), trie, node-based stack.

### When not to use

- When a node needs to be reachable from more than one owner (DLL, shared subtree, graph) — `Box` cannot be cloned or shared.
- When you need shared **and** mutable access from multiple places.

---

## 3. `Rc<T>` — Shared Ownership (Single-Threaded)

### How it works

Reference-counted heap pointer. Multiple `Rc<T>` values can point to the same allocation; data is freed only when the last `Rc` is dropped.

Gives only **immutable** access (`&T`) by default. The count is not atomic, so it is not thread-safe.

### When to use

- A node needs more than one logical owner: DLL (one direction), tree with shared subtrees, parent references, graph edges.
- Pair with `RefCell<T>` (`Rc<RefCell<T>>`) when you also need to mutate shared data — `RefCell` moves Rust's borrow check to runtime (panics on violation instead of a compile error).

### When not to use

- Single-owner chains (plain LL) — pure overhead with no benefit.
- Multi-threaded code — not `Send`/`Sync`.
- Bidirectional links without care — `Rc` on both directions creates a cycle that never gets freed (memory leak).

---

## 4. `Weak<T>` — Non-Owning Reference

### How it works

Created from an `Rc<T>` via `Rc::downgrade`. Does **not** count toward keeping data alive and does not prevent deallocation.

Access via `.upgrade()`, which returns `Option<Rc<T>>` (`None` if the data is already gone).

### When to use

- Breaking `Rc` cycles: DLL `prev` pointer, tree parent-pointer, any back-reference that should not keep the target alive.
- Anywhere you want to observe a node without affecting its lifetime.

### When not to use

- As the primary or owning link — `Weak` alone cannot keep data alive, so the main structural backbone still needs `Rc` or `Box`.
- Simple structures with no cycles — unnecessary complexity.

---

## 5. `Arc<T>` — Shared Ownership (Thread-Safe)

### How it works

Same as `Rc<T>`, but increments and decrements the refcount using **atomic** operations, making it safe to clone and drop across threads. Slightly more overhead than `Rc` due to atomics.

### When to use

- Same shape of problem as `Rc` (shared ownership graphs), but the structure is accessed or shared across OS threads.
- Usually paired with `Mutex<T>` or `RwLock<T>` instead of `RefCell<T>` for thread-safe interior mutability.

### When not to use

- Single-threaded code — pay atomic overhead for zero benefit; use `Rc` instead.
- As a substitute for proper synchronization design — `Arc` alone does not prevent data races; you still need `Mutex` or `RwLock` for mutation.

---

## 6. Raw Pointers — `*mut T` / `*const T`

### How it works

C-style pointer with no ownership tracking, no automatic drop, and no compiler-enforced aliasing rules. Dereferencing requires an `unsafe` block — you take full manual responsibility for validity, lifetime, and freeing memory.

### When to use

- Performance-critical or production-grade structures where `Rc<RefCell<T>>` overhead (extra allocations, runtime borrow checks, refcounting) is unacceptable — e.g. how `std::collections::LinkedList`, intrusive lists, and arena allocators are implemented internally.
- When you fully understand and can uphold memory-safety invariants by hand.

### When not to use

- Learning-stage implementations — debugging use-after-free and dangling pointer bugs has none of Rust's normal compiler help.
- Anywhere a safe abstraction (`Box`, `Rc`, `Weak`) already solves the problem without `unsafe`.

---

## 7. Index/Arena Pattern (`usize` into a `Vec`) — Honorable Mention

### How it works

Not a pointer type per se, but the idiomatic real-world alternative: store all nodes in a single `Vec<Node<T>>` (the arena) and reference each other via plain `usize` indices instead of pointers.

An index is a `Copy`-able number with no ownership implications, so it sidesteps the borrow checker entirely.

### When to use

- Graphs, trees, or any structure with complex or cyclic reference patterns in production code (compilers, game engines, ECS systems).
- When you want arbitrary reference shapes without `Rc<RefCell<T>>` ceremony.

### When not to use

- Pure learning exercises about pointer semantics — it avoids the very concepts (`Box`, `Rc`, `Weak`) you are trying to internalize.
- Structures that need individual node deallocation — arena entries usually live and die with the whole arena, requiring extra bookkeeping like free-lists to reclaim individual slots.

---

## Summary Table

| Pointer | Mutability | Use case | Thread-safe | Cycle-safe |
| --- | --- | --- | --- | --- |
| `&T` / `&mut T` | `&mut` only, one at a time | Traversal, accessor methods (not structural) | Depends on `T` | N/A |
| `Box<T>` | Full (owner controls) | Singly-linked list, simple tree, trie, node-based stack | N/A (move-only) | N/A (no cycles possible) |
| `Rc<T>` | None directly (pair with `RefCell`) | DLL (one direction), shared subtrees, graph edges | No | Leaks on cycles |
| `Weak<T>` | N/A | Back-references: DLL `prev`, tree parent-pointer | No (pairs with `Rc`) | Breaks cycles |
| `Arc<T>` | Via `Mutex` / `RwLock` | Thread-shared trees/graphs, concurrent structures | Yes | Leaks on cycles (use `Weak`) |
| `*mut T` / `*const T` | Manual (`unsafe`) | Production-grade DLL, intrusive lists, arena trees | Manual responsibility | No automatic handling |
| Index (`usize` + `Vec`) | Via arena access rules | Graphs, trees, ECS, compilers — real-world structures | Depends on container | Naturally cycle-safe (no `Drop` chain) |

---

## Quick Decision Path

1. Just reading or writing an existing structure? → `&T` / `&mut T`
2. One owner, one direction? → `Box<T>`
3. Multiple owners, no cycle, single-threaded? → `Rc<T>` (+ `RefCell` if mutable)
4. Multiple owners **with** a cycle (DLL, parent ptr)? → `Rc` + `Weak`
5. Same as above but multi-threaded? → `Arc` (+ `Mutex` / `RwLock`)
6. Need max performance and comfortable with `unsafe`? → Raw pointers
7. Building something production-shaped? → Consider the index/arena pattern instead of pointers altogether
