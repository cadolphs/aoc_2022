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

For game 2, I can re-use lots of the logic before. Basically, writing a `Decorator` struct around the `Cave` 
struct. Now `Cave` becomes `VoidCave`. A `CaveWithFloor` just wraps that. For the construction and all 
other logic we delegate to `VoidCave`, with the exception that `is_out_of_bounds` always returns `false`, 
and `get_square_at` first does an explicit check if we're on the floor (because we don't want to fill our 
`HashMap` with infinitely many x positions, and we don't want to have to guess how many we need, either).

Then we just extract the shared methods into a `Cave` trait and make the `Simulator` generic over that trait.

In the end, I almost got the right answer. Was just off by one because the step where the sand blocks the source 
counts as a step for the answer.