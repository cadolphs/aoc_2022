# Day 03
Okay, now here we can try a different approach: Write everything 
in a messy single big function or two until it works (with tests) and then 
refactor it to better code in a small step by step fashion that doesn't 
intermittently break any tests.

A note here on TDD: When doing that, it's important that the tests aren't 
too narrow-minded. Especially in languages with lots of high-level constructs, 
you don't want to get lost in the weeds of for-loops, attacking a problem 
one item at a time. In hard-core pure TDD, you'd write a test for, say, 
a rucksack that contains only two items, or something like that. 

But with high-level concepts like sets, iterators, etc, that is counterproductive.

So, looking at the problem for day 3, the initial idea is simple: Split the string in two 
halves. Create _sets_ of the characters. Compute the _intersection_ of the sets and the 
single item in that set will be the "appears in both sets" item. Let's go.