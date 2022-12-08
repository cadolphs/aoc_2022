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
