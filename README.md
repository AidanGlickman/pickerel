# Pickerel
A performant, modern, generalized game analysis engine in Rust with alpha-beta pruning (targeting chess).

Authors: Aidan Glickman (aidantg2), Max Fan (myfan2)

(UIUC CS 128 Honors Final Project)

# Goals
Pickerel is a chess engine implemented in Rust. We decided to build a chess engine as we are both avid chess players, and we think that such a project plays perfectly to Rust's strengths.

# Roadmap
We plan to implement a modern chess analysis engine (a la Stockfish) in Rust.
The first thing we plan to do is implement a generalized tree search (with alpha-beta pruning), which will search/evaluate for the optimal moves given any arbitrary position evaluation function.
This will allow us to compare various heuristical position evaluation functions for speed and efficiacy.
After we have a general framework is set up, we'll implement several heuristical position evaluation functions.
Finally, we'll tune and optimize our heuristical evaluation functions as well as implement advanced techniques like NNUE.
If possible, we'll publish graphs and other visualizations detailing the pros and cons of each heuristical position analysis function we looked at.

It is important to note that this framework will not be chess-specific.
Rather, it is a generalized move search/evaluation framework that could, in theory, support other two-player perfect information board games.

# Possible Challenges
There are several challenges that are inherent to a project like this. 
First of all, the success of any artificial intelligence project like this is highly dependent on the creation ot a robust and useful heuristic. 
If we can't determine a way to accurately evaluate the utility of a given position, our algorithm will not be able to efficiently determine the best moves to make. 
Second, this project will be highly dependent on computational efficiency. 
It will be crucial that we optimize every possible component of our project so that our algorithm can check as many states as possible in a reasonable amount of time.

# References
Stockfish
