# Doubly - Linked - List

## Problem description

In a doubly linked list, a node is referenced by both its predecessor and successor (and possibly by the list's head or tail). This means multiple pointers may refer to the same node simultaneously.

But, in rust you cannot have multiple owner of the same data at the same time.

To achieve that in rust we've to use smart-pointers.

`RC` + `RefCell`.

## Pointer needed to create Doubly-linked Node

- `RC (Ref-counter smart-pointer)` : provides shared ownership through reference counting. By itself, it only allows `immutable` access to the contained value `&T`.

- `RefCell` : `RefCell<T>` enables interior mutability. It enforces Rust's borrowing rules at runtime instead of compile time, allowing either:
  - many immutable borrows (borrow()), or
  - one mutable borrow (borrow_mut()).

## Why `Rc<RefCell<T>>` together ?

- Rc → "multiple nodes can point to me" (shared ownership)

- RefCell  → "I can still be mutated even through a shared reference" (interior mutability)

## Node Structure looks like this

```Rust

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
    prev: Link<T>,   // now A can be owned by both Head and B's prev
}

```

## One caveat to keep in mind

`Rc<RefCell<T>>`will create reference cycles

```
- node A holds Rc to B, 
- B holds Rc back to A via prev. 
```

**Neither will ever drop.** You'll need `Weak<RefCell<T>>` for one direction (typically prev) to break the cycle. Worth tackling once the structure is working.

### So Better to impl like

```Rust
type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
    prev: WeakLink<T>,
}
```

## Important Findings:

- What does `take()` do?
Calling `.take()` on an `Option` takes (moves) the value out of the `Option`, leaving it as `None`. For example:

```Rust
let mut x: Option<i32> = Some(5);
let y = x.take();
// y = Some(5), x = None
```

- Key difference in these two code blocks:

**Case 1: Taking directly from a struct field**:

```Rust
let ptr: Option<Rc<RefCell<Node<T>>>> = self.head.take();
// after this: self.head is None and ptr has the original value of self.head
```

  - Ownership of the value is moved out of `self.head` into `ptr`.
  - `self.head` is now `None`, so the node is no longer referenced by the list head.


**Case 2: Taking from a local clone (not affecting struct field)**:

```Rust
let mut ptr: Option<Rc<RefCell<Node<T>>>> = self.head.clone(); // just clones the pointer
let target = ptr.take(); // ptr becomes None, but self.head is untouched
// self.head remains unchanged. Only the local variable is emptied.
```

  - Here, only the local `ptr` variable is modified; cloning does not affect the original `self.head`.
  - `self.head` still holds its value (with its reference count unchanged).

In summary:  
- `.take()` on a struct field (like `self.head.take()`) removes the value from the struct and replaces it with `None`.  
- `.take()` on a local variable just sets that local variable to `None` — it does not affect the source field or variable you cloned from.
