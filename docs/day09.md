# Day 09
This one shouldn't be too bad. Just need to create the right data structures so the code is readable... 

So all we need is a `Vec2D` datastructure. Then we overload the + and - operators for convenience. Then it's just 
about keeping the structs clean. 

For part 1, the rules are easy enough. Now for part 2, we have a few new situations. The head still can only move 
in straight lines. But because of the movement rules, knot 1 will now be able to move diagonally, which means 
that new situations can occur. So, we need to update the "get new direction rules".

And then we get to play with generics and traits. In a way this is the "strategy pattern". The `GameTracker` struct 
doesn't need to know how the "situation" is handled. All it needs to be able is tell the situation about a move to make 
and then query it for the new tail position. So, a bit of refactoring and we can just use the same tracker struct for 
both part 1 and 2.

One thing I'm never sure is whether the "idiomatic" way is for a method to _consume_ `self` or take a mutable reference on `self`. 
There's probably no right or wrong here, just a matter of taste; though in certain situations it might indeed make sense 
to prefer one over the other.