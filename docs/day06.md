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