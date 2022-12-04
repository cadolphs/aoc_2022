# Day 04
Main challenge here for now is probably the parsing. With parsing, there's three options 
in growing complexity and power:

1. The easiest way is to just iterate through the string character by character, or do some 
very basic string processing. 
2. Next up, we have regular expressions and matching them. 
3. Finally, we have the option to write down a formal grammar in a format like EBNF. That's often how 
a programming language's syntax is formally defined.

Anyway. Here I feel a simple string manipulation should suffice.

The idea then is to parse each line into a pair of ranges, then see which ranges contain each other.