# Shellfish - [ikhovind.no](https://ikhovind.no)

This is a multi-threaded bitboard chess engine using a minmax algorithm with alpha beta pruning written entirely in Rust. 

The engine and a simple JavaScript frontend are deployed on Google Cloud, hosted on https://ikhovind.no

## Resources

Resources that have been especially useful during development have been:

   * [The Chess Programming Wiki](https://www.chessprogramming.org/)
   * [Sebastian Lague's video on Chess Programming](https://www.youtube.com/watch?v=U4ogK0MIzqk) 
   * [The YouTube channel Logic Crazy Chess](https://www.youtube.com/logiccrazyguide)

## Dependencies

The frontend largely relies on [Chessboard.js](https://chessboardjs.com/) in order to draw the board and pieces. 
Showing the user which moves are available is done through [Chess.js](https://github.com/jhlywa/chess.js), but all calculations of moves made by the AI is only made on the backend.

The backend relies on a number of non-chess related dependencies, all of which can be seen in the Cargo.toml

## Improvements
There are many areas of improvement for the chess engine itself. First of all it could be optimized further, primarily in the engine library, but the move generation could also make good use of more paralllel calculations.

The engine lacks the ability to see draws by repetition, which means it often draws winning endgame positions, this is a feature that could be easily added after some form of hashing functionality has been added.
The primary hashing feature to add would be Zobrist hashing, this would greatly strengthen the engine by allowing faster computation and therefore deeper lines.

The evaluation function should also be tinkered with, there is no guarantee that the piece values or piece square value tables used now are ideal.

Finally, the engine should implement iterative deepening, so that it can be given a move-timer and simply a static depth of four as it is now. This would of course make it more enjoyable to play against, but also strengthen its endgames. A depth of four has been chosen purely to make it tolerable to play against in the early game, while the endgame is less computationally intensive and is therefore limited by the chosen depth.
