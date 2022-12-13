# Day 13
Nested data structures, yay. And lexicographic ordering. The parsing will be interesting but the 
logic should be straightforward.

The fact that we put the recursive part of the type definition into a `Vec` nicely takes care of the 
fact that you can't have a "naively" recursive data structure in Rust:

```rust
// Doesn't work: How big is a tree anyway?
enum Tree {
    Leaf,
    Node(Tree, Tree)
}
```

Now for the comparison, we can build this up bit by bit. First we'll do the "happy" case of two 
flat lists.