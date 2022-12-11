# Day 11
Monkey business! Parsing shouldn't be _too_ bad. But let's get the logic down first.

Again, the logic isn't overly hard. Just one thing I already know: The Rust 
borrow-checker might make a overly simple implementation impossible, because 
you can't have one Monkey hold mutable references to the other Monkeys.

Anyway. There's a few design decisions to make. How general do the operations have to 
be, for example? Should they be represented via an enum or via a closure / 
function object? Here we go for simplicity.

Before writing more implementations for the `Monkey` struct we'll want to think about how the 
`MonkeyGame` will be orchestrated. The idea is that the game will ask a monkey to do its turn. 
Then, rather than having one Monkey struct tell another Monkey struct to update its items, 
it will return a list of _messages_, containing of an item's worry level and the target monkey. 
That could be a very simple struct.

A quick thought. In the monkey's `take_turn` method, I'm mixing return values and side-effects. 
That's considered a violation of the "command-query separation" design principle. However, 
this principle isn't ironclad (cf. methods like `pop` on a stack data structure). Here, I'd argue 
that the side effects aren't _surprising_.

Next up the parsing...