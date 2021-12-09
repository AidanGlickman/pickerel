# How to use Pickerel

To run pickerel, simply build the project through Cargo, and run the main executable. At this point you will be prompted to input a FEN string. This is a standard representation of a chess position. After inputting your FEN string, the engine will reply with its evaluation of the position using both a minimax based engine with no pruning that runs on one thread, and one that use the [Simplified ABDADA](http://www.tckerrigan.com/Chess/Parallel_Search/Simplified_ABDADA/) parallelized tree search algorithm, which will spawn many threads and search the tree in parallel while also using alpha-beta pruning.


```bash
cargo run
[input FEN to stdin]
[input FEN to stdin]
[input FEN to stdin]
```