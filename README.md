# othello_multimcts
Multithreaded heuristical MCTS-based [Othello](https://en.wikipedia.org/wiki/Reversi) engine

[Compatible browsers](https://developer.mozilla.org/en-US/docs/WebAssembly#browser_compatibility)
for online [Demo](https://xumarcus.github.io/othello_multimcts)

# Features
* CLI interface with SIMD support
* GUI frontend with WASM speed up
* Supports multithreading for time-consuming heuristic
* Late game solving / pruning
* No opening book

# Note
* Generally MCTS performs better when branching factor is large
* However there is no simple admissible heuristic for Othello
* Compare with [NTest](https://github.com/weltyc/ntest)
or [Edax](https://github.com/abulmo/edax-reversi)
to assess strength.
* Idea comes from [UCThello](https://github.com/OMerkel/UCThello)
which this implementation beats.

# Todo
Adapt this program for Go.
