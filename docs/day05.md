# Day 05
Yikes. This one is going to be all about actually reading the input and parsing it. 

We can "cheat" a bit by not making the program overly generic. It seems that _my_ 
puzzle input has 9 stacks. That means each stack id is a single digit, so that each 
column in the input has the same width. What we have then is 3 characters per column 
separated by a single whitespace. That means we can figure things out with simple math.

Let's hack things together slowly in the test module.

Okay, reading a move line is a straightforward regex thing. Now what about reading a 
crate line? Ideally that would yield an iterator over pairs (stack / box)? Though 
with the matching it seems there's some "lifetime" troubles. So, here it might just 
be more practicaly to just write a for-loop that checks what's at the positions...

Next up, we'll have to actually implement the number of crates. Let's _not_ have that be 
a primitive `Vec`, because it might have some issues with mutability. So, to build the 
stacks one by one, we append to one end of a double-ended queue. Then, to apply a move, 
we pop from the _other_ end of the queue. That makes it so we can read the crate lines 
top to bottom.

Now I'm using for-loops a bit more; this is because it seems more idiomatic when calling 
methods on mutable structures. I like iterators to be more or less side-effect free.

For part 2, all we need to do is remove `num` items, put them on a stack, and then 
put them to their destination by popping from that stack.