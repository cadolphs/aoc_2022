# Day 14
Why haven't I discovered the `nom` crate sooner? Let's try it out for this one right now.

Here I'll actually start with the parsing. Might make it easier to test the logic. And 
because of the way `nom` works, we can build up the parsing nice and incrementally. Excited!

Okay. Parsing was super nice with `nom`. Then the logic... a bit more messy. But trying to use 
some good idioms here and there.

One observation I had was that I shouldn't recompute the path of the sand from the source in each step. 
Instead, I can just backtrack to the last air square on the previous sand's journey. A simple stack should 
do the trick.

And then we just run a loop until we're finished. Could probably do something nicer with iterators but meh.
Great: Got the right answer for the actual puzzle input on first try.
