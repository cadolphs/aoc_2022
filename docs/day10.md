# Day 10
A computer, yay! So. I suspect that subsequent days will add on lots of functionality. That means we should keep the 
code for today _flexible_ without _overdesigning_. Like, I don't know yet how the CPU and clock and everything will 
be used later down the road. Maybe some fancy async stuff will be required. But it's overkill to implement that right now. 

So, do the simplest thing that works without hardcoding things.

For _this_ day, it seems to me we can have a simple loop that consumes the instructions and updates the register appropriately 
and pushes them to a vector so we can later check what was what.

