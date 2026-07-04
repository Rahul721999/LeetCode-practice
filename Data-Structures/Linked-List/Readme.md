# Linked List implementation

## How NOT to make Linked List in Rust

This is not allowed in Rust as it requires to know the size of the struct at compile time

```Rust
    pub struct LLNode{
        pub val: i32,
        pub next_node: Option<LLNode> //! inifinite size error
    }
```

### How it will look like in Heap Memory

```bash
    +---------+
    | val=10  |
    | next -> entire Node
    |   +---------+
    |   | val=20  |
    |   | next -> entire Node
    |   |   +---------+
    |   |   | val=30  |
    |   |   | next -> ...
```

## How to prevent Infinite size Error at compile time ??

Ans: We can use the Pointers provided by rust standard library.

### We can use `Box<T>` pointer in this case. Why ?

- Diff pointers in rust sovles diff `Ownership` problems.
- `Box<T>` pointer ensures `single Owner` of value: `T`.
- In our case `T=LLNode`; So every `LLNode` will have only one `Owner`;
- So when the `child node` is dropped, everything below it will also be dropped.

### Why not using `&T` ref ?

Ans: Because it's just a reference. We dont own the data. Any point of time Rust asks:

- Who's the owner of the Node ?
- How long do they live ?


### Why not using `RC` Ref Counter? — Shared Ownership, Single-Threaded

Ans: Ref Counter smart pointers let's you have multiple owner of the same data. And keep counts of the owner. As soon as no owner left, it drops the value. `Node is freed when counter reaches 0`.

- To build a simple LinkedList we dont need multiple owner.
- As one `child-node` should have only one `Parent Node` owner. And that `Child-node` will be the `parent-node` of another child. And so on..
- So there's no need need of `RC`

#### Where it's needed

Anywhere your data structure's shape requires more than one logical owner for the same node:

- Doubly-Linked List (naive first attempt): a node is pointed to by both its predecessor's next and its successor's prev. Two incoming pointers = two "owners" conceptually.
- Trees where children need to reference their parent, or where multiple parents might reference the same child (e.g., a DAG-like structure, or a trie with shared suffixes).
- Graphs in general, where a node can have arbitrary in-degree.

### Why not using `ARC<T>` ??

Same like `RC`, `Arc` also allow us to have multiple owner of a single data. But this is for multi-threading. So we dont need this too.

