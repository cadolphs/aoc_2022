# Advent of Code 2022 in Rust
I'll try to find some time between work and taking care of the kids. Now. I think the _ideal_ language for 
quick solutions to AOC is Python. Dynamic typing and a crazy large ecosystem of packages. But where's the fun in that?

Rust is a fascinating language in its own right. Compiled and "low level" in the sense that you control the memory and 
not some garbage collector. But also with high-level ideas. Enums are cool. Traits are cool. The ownership model of 
memory management is unique and fun to play with. So, I'll try to use this opportunity to play around with idiomatic 
Rust.

## Getting the input
I don't like to copy-paste the input files. Instead, I'm using a crate (aocf) that'll get me access to my personal 
version of the puzzle input. It's based on my session cookie which I store as an environment variable in gitpod.

