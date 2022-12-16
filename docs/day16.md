# Day 16
A proper graph algorithm challenge! Wow. So I tried thinking about this in very general terms and found it very 
challenging to find a promising _ansatz_. Ideas like backtracking and dynamic programming came to mind, but 
the state that would need to be carried through the recursions grows combinatorially. In such a case, it makes 
sense to inspect the puzzle input a bit closer. Here, I notice that many nodes have flow-rate 0 and therefore 
only act as connections. 

So let's start with parsing and count definitively how many non-zero flow-rate nodes we have.

