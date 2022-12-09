# Day 08
Ah, I was expecting one of these map-pased puzzles to show up eventually.

This looks like a good excuse to familiarize myself with the `polars` crate--Rust's equivalent to Python's `pandas`. 
Or the `ndarray` crate, the equivalent to `numpy` (well, in terms of array handling.)

First step is to figure out how to read the input :D 

Looks like with `ndarray` there's not a simple "from 2d iterator" function; that's because ideally there'd be some 
straightforward moving involved, and with iterators you don't necessarily know their size. Anyway, reading the digits 
of the input into a `Vec<Vec<i8>>` is straightforward enough and then we just do a 2d loop. I have to remember that 
in Rust, unlike in Python, there's nothing inherently bad or poor-performative about for-loops. With Python, you want to 
push iterative code down into the underlying (C-implemented, mostly) libraries.

Anyway. Now for the actual puzzle. Let's start first about how to figure out which trees are visible from the left, and ignore 
everything else. Well, we should compute the "cumulative maximum" along each row. Then we need to shift everything over a bit 
and then compare the sizes. Let's think about a single line.

`25512`

And the cumulative maximum line would be 

`25555`

What we need to compare now is the shift:

```
25512
 25555
```
This tells us that the first 5 is visible from the left and no other trees in that 
row are visible from the left. Let's try that...

Well. First I need to compute the cumulative maximum. Couldn't find a nice easy way to map. But then again, loops aren't that hard.

Next it's about setting different axes and directions. The axes are easy; just pass those as an arg to the `axis_iter`. What about 
direction?

Hah. This is where a statically typed language becomes a bit more awkward. Basically, I wanted to use in the for loop either the 
"forward" or the "reverse" version of the enumerated iterator over rows: 

```
let row_iter_enum = row.iter().enumerate();
if let Direction::Reverse = direction {
    row_iter_enum = row_iter_enum.rev();
}
```

However, that doesn't work! Because those iterators will have different types, and you can't assign variables of different types to each other! 

Now for the loop, all we need to know is that both iterators implement the `Iterator` trait and will yield items of type `(usize, &i8)`. So we need 
a dynamic trait type. And because you can't know the size of a dynamic type at runtime, you need to put it in a `Box`.

Cool stuff. Now let's use the cum_max arrays to figure out which treas are visible from which side.

Okay, now learning about how to deal with applying things to arrays. Seems like "<" isn't overloaded for arrays, so I have to write it myself. That 
brings up the `Zip` structure. Looks like this is where all the cool "lockstep elementwise" stuff is happening. I might even be able to rewrite the 
"cumulative maximum" function with Zip...

Now as the last step, there's lots of tricky array shuffling. But this is good practice and I'm getting the hang of it, maybe. By using unit tests on the 
aoc test input, I made it so that once I actually ran everything on the puzzle input, I got the right answer on first try!

On to part 2. Okay, more such logic. Though I think we can (almost) use the same approach. I want to avoid doing this individually for each tree.