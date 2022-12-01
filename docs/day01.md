# Day 01
The first problem is usually quite easy. The main task is to parse the data file in the correct way and do some 
very simple transformation. In Rust, I'd like to use _iterators_ for that purpose.

So, by splitting the input on a blank line, we'd get an iterator over the "chunks" of each elf. By splitting 
_those_ chunks, we get the calories per elf. Then we're asking for the max over the sum. We can hack this together 
at first, and then do a cleaner job with iterators and structures. Let's go!

Okay. Inside the test module of day 1 I just hack around (and set the tests to run automatically on change via 
`cargo watch -w src/ -x test`) and just play around with chaining iterators. Seems to work, so now I can just write 
the code properly.