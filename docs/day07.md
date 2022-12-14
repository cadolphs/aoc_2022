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

Now that the parsing works, what about the actual algorithm? Let's start with a messy loop, then refactor later :)

So. The loop I created worked for the test input on the aoc site, but I got a wrong result for the actual puzzle input. It says my number is too low. 
I assume that indeed we can have _sub_-directories with non-unique names, which messes up my algorithm. That just means I should tweak the way the 
directory tally is computed.

Okay. Just as I suspected. So now in the `active_dirs` vec, when adding a new entry, it needs to be prefixed with the "fully qualified path". So, this 
is a good example where a _simple_ test case (the input on the AOC problem page) doesn't catch _all_ the subtleties of the true input.

Now that this is fixed, we can easily solve part 2 as well, with iterators and filters. Let's go.

And finally, the code is a big mess because I hacked everything together in one loop. We can probably refactor that and clean it up. We have the tests that 
will catch if we mess up anything too big.

First, the device module is getting too big. Let's split it into separate modules.

## Refactoring
The first step is to just move `device.rs` to `device/mod.rs`. That shouldn't change anything else. But now we can create extra files to house the various things. So now
we'll create a file for the message parsing and one for the filesystem.

To do that "gracefully" instead of in one big go, I first just _copy_ all the message parsing stuff into `messages.rs`. Then I _move_ the tests over. Then, I can replace the 
originals in `mod.rs` with just imports (and re-exports?) of the stuff in `messages.rs`.

Phew. Then, I encapsulated all those collections in their own `ParseHelper` struct. That then allowed me to clean up the logic quite a bit. I didn't like this awkward coupling 
where depending on the "reading mode" I advance to the next line or not. That then also meant I didn't need to use different logic for processing lines or commands depending on what 
came before. The resulting code is pretty concise.