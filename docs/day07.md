# Day 07

Okay. Parsing will be a pain. But at least for part 1, let's not over-engineer. The _first_ thought when reading this 
is of course "Oh, this is a directory system. That's a __Tree__ structure". And of course those structures are a bit messy to 
implement with Rust, because of the special ownership and borrowing rules. Basically, Rust doesn't quite like tangled webs of things 
pointing at each other. For good reason, mind you, but it makes implementing these standard idioms hard.

Anyway. At least for part 1, we don't _need_ to put all the files and directories into a nice tree structure, because the puzzle input 
is doing the traversing part for us already. We just need to keep a tally of what we've seen in the directories so far. My idea is that 
all we need is a `HashMap<String, u64>` that keeps a tally of what we've seen so far in terms of files. As we traverse, we keep track of 
the current (linear) path of directories we're in. One thing that's not entirely clear is whether directory names are unique across 
sub-directories. In a _real_ filesystem of course you can have a directory `foo` containing directory `bar`, but also a directory 
`baz` containing a directory `bar`. So we might need to use the full path to keep track.

Let's start with a simple command crawler and see where we get.

So. First, we need to parse the input. For now I'm using a single enum that keeps track of everything a line can be: A command or an entry. 
And then for each command I have an enum variant as well as for a file entry and a directory entry. Tests check that the parsing works as intended.