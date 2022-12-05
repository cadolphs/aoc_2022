# Day 05
Yikes. This one is going to be all about actually reading the input and parsing it. 

We can "cheat" a bit by not making the program overly generic. It seems that _my_ 
puzzle input has 9 stacks. That means each stack id is a single digit, so that each 
column in the input has the same width. What we have then is 3 characters per column 
separated by a single whitespace. That means we can figure things out with simple math.

Let's hack things together slowly in the test module.