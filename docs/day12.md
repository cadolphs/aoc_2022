# Day 12
Ah, path-finding. Instead of constructing a graph and just using a library, 
here it might be fun to produce Dijkstra's algorithm from scratch.

Indeed. And this time I didn't want to bother too much with writing custom structs. It's all in 
a few functions and one big loop. But it's actual Dijkstra :)

For part 2. I know there's a generalization of Dijkstra that helps compute _all_ shortest paths between 
all nodes. But that's overkill for me. Probably. Plus, I don't care about _all_ possible ends.

Just need to extract all possible starts. That part is easy enough. Now the issue is here that apparently, 
sometimes the path can't be found. That makes sense! There might be squares from which it's impossible to 
reach the end. Stuck in a local minimum or whatever. 

This is a perfect use for rust options. Instead of panicking when a path isn't found, as I did for part 1, 
I can just return `None`. 

Then for part 2, I first compute all the possible starts. Then I compute the shortest path length to the end 
for each of the starts, as an iterator. And then I just have to drop all the Nones before computing the minimum.

The runtime isn't the greatest. Smarter algorithms can be devised. We're still repeating a lot of computation. 
What would be better is: Create set of all the possible starts. Then pick one and compute the shortest path to the end. 
Now, the cool thing is that any _other_ possible start that would be on that path would automatically be the shortest path 
as well. In short, if G is the shortest path from A to B, and C lies on path G, then, starting from C, G is also the 
shortest path from C to B. (Simple proof via contradiction. Try it at home :D ).

Now, in part 1 I didn't actually keep track of the path...

One other note: Here, _all_ edges have the same "length" of 1. So we don't need to use the `decrease_priority` function 
of the queue. In full-on Dijkstra, what can happen is that _after_ we already added a node to the queue, we find an 
even better path to that node. 