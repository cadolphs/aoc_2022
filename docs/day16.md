# Day 16
A proper graph algorithm challenge! Wow. So I tried thinking about this in very general terms and found it very 
challenging to find a promising _ansatz_. Ideas like backtracking and dynamic programming came to mind, but 
the state that would need to be carried through the recursions grows combinatorially. In such a case, it makes 
sense to inspect the puzzle input a bit closer. Here, I notice that many nodes have flow-rate 0 and therefore 
only act as connections. 

So let's start with parsing and count definitively how many non-zero flow-rate nodes we have.

Phew. Running out of time and I got other things going on. So here's my current thinking that I might come back to:

After reading in the "raw" graph, we realize that we don't care about the 0-nodes. We instead compute the 
_transitive closure_ over all nodes with non-zero flow-rate. That gives a complete graph with edges whose weight 
indicates how many steps it would take to go from one node to another. The intermediate nodes don't matter.

That simplifies things quite a bit. Because it means if in _that_ graph we travel to a node, that only makes sense 
if we then open that node's valve. 

And I think with that, we can do some nice backtracking with early stopping. We do recursion. 

Okay. Giving up on this one :) The "all-pairs shortest distance" idea is right on, and then we can use dynamic programming 
where the recursive argument is a _bitmap_.