# Day 06
Yay, no tricky parsing today!

First, the text mentions that the __device__ will have many interesting functions. So I suspect over the course of the event we'll be 
adding a lot of stuff to it. Let's therefore put the code for the device right into its own source file.

Now, we don't want to overengineer anything just yet. Good software design doesn't mean fancy patterns everywhere. It means code that, 
once new requirements come in, can respond gracefully to those new requirements. We want to avoid premature abstraction. 

For example, we don't yet know what the device will do with the rest of the message. Let's just do the minimum reasonable effort.

This sort of processing of a stream of characters reminds me of a 2nd year comp sci course (Finite Automata) and of grammars. For one, 
I don't think there's a _straightforward_ way to just use a regular expression that says "4 characters that are all different". At the 
very least, you need something like back-referencing.

How would such a regex look like? `([a-z])([a-z&&[^\1])([a-z&&[^\1\2]])([a-z&&[^\1\2\3]])`

I tried hacking around with that, but it seems you cannot use back-references in capture groups. I'm not surprised here. The _language_ 
defined by what we're trying to match is not _regular_, and therefore cannot be parsed by a standard regular expression. Seems like 
some very ugly messing around with look-ahead and look-behinds _might_ do the trick. But at this point it's much easier to use some sort of 
state machine for this.

Okay. Did some hacking right in the unit tests. Rust's pattern matching is actually quite nice for (small) state machines. We write an 
`enum` that knows what our current state is. At the beginning we haven't seen any characters yet, so the state is `Empty`. And then we can 
be in any state of "how many unique characters were there for now". Writing the transition function becomes quite easy with the matching then. 

For example, if we have seen two unique characters so far, say `ab`, then there's three possible outcomes: The next character is new, so we move to the 
`Three` variant of the enum, as in `abc`. Or the next character is the same as the most recent character, in which case we move all the way back to the 
`One` variant (`abb` means now `b` is the only single unique character). 
Or the next character is different from the most recent character but matches the second-to-last character, as in `aba`. Then we move to the state where 
we've got two unique characters but now in different order, `ba`.

Okay, so that all works. But of course now they throw us a wrench in part 2 where we have to do 14 distinct characters. A manually written state machine doesn't quite work there 
any more. Instead we'll need to use a _collection_ to keep track of what we've seen so far. Now, naively we can just look at the string in 
overlapping windows of length 14 and check if there's 14 unique characters. But there should be a better way. This now reminds me of the _scanline_ algorithm for 
word search. Here's how it works:

1. We keep track of the (potential) start position in `start`.
2. We have the search position in `head`.
3. We maintain a queue (or dequeue?) of the letters between `start` and `head`.
4. When we advance `head`, we compare the new character against the collection of letters between `start` and `head`.
5. If the new letter wasn't seen before, we keep going until we hit 14 letters.
6. Otherwise, if it _was_ seen before, we need to advance `start` all the way to after the _last_ occurrence (most recent) of that letter.

Well. Turns out the `head` isn't even needed because that's implied in `letters.len()`. Then it was just about using the unit tests to avoid off-by-one errors and 
to implement this algorithm. The nice thing is that we can then also get rid of all the `enum` and `struct` stuff, because we can just reuse the algo from part 2 for 
part 1 as long as the "number of uniques" is kept as a variable. Yay!