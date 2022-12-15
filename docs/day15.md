# Day 15
Geometry! This seemed more intimidating than it might turn out to be.

First of all, looking at the puzzle input, a _brute force_ approach where we 
just iterate over all the squares and check individually, seems wildly 
unfeasible. It might _seem_ to be linear in the input, but it's not. The 
actual amount of sensors is quite small, but the number of squares they cover 
is gigantic. I assume that for part 2, we'll have to deal with the full 2D 
geometry, where we'd then get billions of squares to check. Not fun.

Instead, first we start with part 1, and ask how we can do it in a way that's 
at most polynomial in the number of sensors (and beacons). 

In 1D, along a line, that's easy enough to see: Each sensor-beacon pair defines 
an _exclusion zone_ in which we know there's no beacons. Each exclusion zone 
will have a (possibly empty) intersection with the horizontal line we're interested in. 

Now all we need to do is compute all those intersections (simple math), and then 
_combine_ all those intersections to get the total amount of excluded squares on the 
y-line.

So. First we write an `Interval` struct. Then I wrote a `subtract` method. But really 
that's probably too cumbersome. Instead, I should have an `Intervals` struct that 
maintains a view of disjoint (and ordered) intervals.

We'll TDD the adding of new intervals.