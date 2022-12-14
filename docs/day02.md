# Day 02
Another day that seems algorithmically simple. Here we can look into a few _good principles_ of coding, though. One that 
jumps to mind immediately is the DRY principle: Don't repeat yourself. Every piece of _knowledge_ should have a single 
source of truth within the codebase. 

What that means in this case is that the translation from letter to hand shape ("A means Rock") should happen at exactly 
one place in the code base. Without having seen part 2 of today's challenge, I suspect that this will become especially 
important when we look at the "X, Y, Z" meanings. Right now we only have a hypothesis for what hand shape those letters 
refer to. For part 2, we might have to change it. If we have a bunch of code and logic that explicitly uses "X" and 
_implicitly_ has it mean "Rock", this will be a very burdensome change.

So, Rust enums seem great for representing hand shapes. Then we just need to provide a single function that turns a character into 
a hand shape. Let's go!

With the enum ready to go, we can now also think about the game. A game outcome is another good candidate for an enum. 
Then we can add a function that takes _your_ HandShape and battles it against an opponent, and returns an outcome.

With that done, there'd now be the _scoring_ function. We _could_ implement those as methods of the handshape and game outcome 
structs but conceptually I don't like it. At least not for the hand shape. The scoring is something that depends on the 
particular rules of the game.

Now with scoring implemented, we need to parse the whole list. It's nice that we have a `FromStr` for the hand shapes. Let's see if we 
can use that to directly convert tuples...

Phew. Lots of wrangling the iterators, parsing, and collecting. Getting better at it :)

Next. For part 2. Aha. So what's changing here is that the second column doesn't refer to hand shapes but to game outcomes. Easy enough 
to implement `FromStr` for the game outcome.

After that, the parsing itself works the same way. And then we just need a function that turns the intended outcomes of the game into the 
hand shape we should be throwing (because we need to know that for the scoring). Pattern matching does the trick here.