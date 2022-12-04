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

There's nothing too crazy difficult here. The only "trick" I used is to put the `Range` struct into its 
own module. That's because _visibility_ of a struct is on a per-module level. That way, I have control over 
the fields of the struct. The idea is that, when constructing the `Range` via `new`, I check if the range 
is valid (`low <= high`). But if the fields were accessible from the outside, I can't guarantee that invariant.

Anyway. The rest is just again simple parsing and iterator stuff. Maybe one cool thing to notice: When you have an 
iterator that returns items of type `Result<T, E>`, you can _collect_ that into a `Result<Vec<T>, E>` directly.