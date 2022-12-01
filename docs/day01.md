# Day 01
The first problem is usually quite easy. The main task is to parse the data file in the correct way and do some 
very simple transformation. In Rust, I'd like to use _iterators_ for that purpose.

So, by splitting the input on a blank line, we'd get an iterator over the "chunks" of each elf. By splitting 
_those_ chunks, we get the calories per elf. Then we're asking for the max over the sum. We can hack this together 
at first, and then do a cleaner job with iterators and structures. Let's go!

Okay. Inside the test module of day 1 I just hack around (and set the tests to run automatically on change via 
`cargo watch -w src/ -x test`) and just play around with chaining iterators. Seems to work, so now I can just write 
the code properly.

Next up we have part 2 of day 1 where now instead of just the max, we need the top 3. We _could_ of course just throw 
it into a vector and sort it. But that seems a waste. Instead, if you want something where it's algorithmically efficient 
to iterate over the top items, you'll want a heap data structure. In essence this would be a _priority queue_. 

Well. We can actually do even better than using `BinaryHeap`. Given that we know ahead of time that we only want the top three, we 
don't need to put _all_ the items in a heap. Instead, we just do a single pass over the data and keep track of the top three. 
That could either be done via a for-loop, or, much cooler, via the `fold` method of iterator. The `fold` method takes in an 
initial value and a function. It then iterates over the items in the iterator and updates the _accumulator_ with the result of 
calling the function on the current value of the accumulator and the next item. Maybe that sounds complicated but if you check out the 
example it's not that difficult: Here we initialize the accumulator with `(0, 0, 0)`. To process the next item, we just check where 
the next item fits within the current top three.

Quick thought: If we had to deal with a _very large_ dataset _and_ we'd be asking for _a lot of top items_ (say, the top 100,000 of 
a data set with 100,000,000,000) items or so, we could combine the ideas. The accumulator would not be a simple tuple but instead a 
double-ended priority queue. To update the accumulator with the next item, we check if the next item is larger than the current minimum. If it is, 
we pop the minimum and push the new item. Otherwise we do nothing.

## Cool Rust features here
### Iterators
Iterators and their chaining is very powerful and idiomatic. In "the old days" you'd write a for-loop for these sort of things. Now you 
just chain commands together that transform and iterator. This is more expressive because the code tells us _what_ we're doing, 
whereas old-school loop code would get lost in the details of _how_ we're doing it.

### Traits, trait bounds, generics...
Another cool feature on display is the type system, with traits and all that. For example, 
`let heap: BinaryHeap<i32> = sums.collect()`.

Why is this cool? Well, the _usual_ way to think about creating a collection from an iterator is that the collection should have a constructor that 
takes in an iterator. Something like  `let heap: BinaryHeap<i32> = BinaryHeap::from_iterator(sums)`. Because this is such a common idiom, there's 
a _trait_ for that: `FromIterator`. 

Rust then just adds some extra convenience: All iterators provide the `collect` method, which is really just using trait bounds to say: "To collect 
an iterator into a collection of type `B`, where `B` must implement the `FromIterator` trait, just call `B.from_iterator`.

There's a couple other "pairs" of traits that help with conversion. For example, with any type that implements the `FromString` trait, you can just 
call a string's `parse` method instead.

### The Option enum
What's up with that `unwrap` stuff and with the `Some(13)` in my test? Well, think about the heap's `pop` method. It's supposed to remove the top 
element and return it. What if there's no element? The two traditional ways of dealing with that would be: Raise an error (throw an exception) or return a 
null value. Python for example would raise an `IndexError: pop from empty list`. 

Rust _does_ return a sort of null value when `pop` is called on an empty heap. But Rust is also a strongyl typed language. If it wants to return something like `None` 
when `pop` is called on an empty heap, it can't return `i32` when called on a heap filled with integers. 

Instead, the return type of `pop` is `Option<i32>`. What's that? It's an enumeration. The definition looks something like 

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

An enum has _variants_. The cool thing about Rust enums is that variants can have a decent amount of structure to them. The `Option` enum has two variants. `None`, and `Some(T)`, where 
`T` is a generic type parameter. So in our example, `Option<i32>` can have values like `None`, `Some(13)`, `Some(42)` or whatever. This solves the issue that `None` and `42` have different types whereas 
a function needs to have a single return type.

With the `unwrap`, we now take an option-typed value and say: "If it's the `Some`-variant, return the data contained in it. If it's the `None`-variant, abort with an error". Of course we could also choose 
more sophisticated error handling but for our purposes here that's enough.