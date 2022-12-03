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

Okay. That wasn't so hard in the end. The `Result` stuff might seem a bit awkward but that's just 
Rust making sure that we don't have unhandled surprises.

Now for part 2. Let's first focus on a single group of three elves. The idea is the same, using 
the intersection feature. Just using some more iterators to that effect. 

The hard part was figuring out how to iterate over chunks. It seems the `chunks` in itertools 
doesn't quite do what I need it to do, but the `tuples` method does. I think maybe the idea is 
that with `chunks`, you can actually process them in parallel, whereas with `tuples` it's 
understood that you still want to go through it sequentially.

Anyway. Once that was figured out, it's again reasonably straightforward. I'm wondering though if there's 
a nicer way to skip the various "unwraps" and consolidate them.