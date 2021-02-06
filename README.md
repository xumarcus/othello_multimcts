# othello_multimcts
Multithreaded heuristical MCTS-based Othello engine

[Compatible browsers](https://developer.mozilla.org/en-US/docs/WebAssembly#browser_compatibility)
for online [Demo](https://xumarcus.github.io/othello_multimcts)

# Features:
* CLI interface with SIMD support
* GUI frontend with WASM speed up
* Supports multithreading for time-consuming heuristic
* Late game solving / pruning
* No opening book

# Note:
* Generally MCTS performs better when branching factor is large
* However there is no simple admissible heuristic for Othello
* Alpha-beta based ones like [NTest] (https://github.com/weltyc/ntest) perform better
