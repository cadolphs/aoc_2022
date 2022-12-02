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