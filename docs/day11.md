# Day 11
Monkey business! Parsing shouldn't be _too_ bad. But let's get the logic down first.

Again, the logic isn't overly hard. Just one thing I already know: The Rust 
borrow-checker might make a overly simple implementation impossible, because 
you can't have one Monkey hold mutable references to the other Monkeys.

Anyway. There's a few design decisions to make. How general do the operations have to 
be, for example? Should they be represented via an enum or via a closure / 
function object? Here we go for simplicity.