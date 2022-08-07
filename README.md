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
The engine has many areas of potential imporvements. First of all, the engine does not take draws by repetition or the 50-move rule into account, meaning that it often draws winning endgames, or loses where it would be able to force repetition. For the sake of execution speed, at least three move repetition should be handled by Zobrist hashing, which could also be used to greatly improve the general execution speed, especially in endgames.

The program could also be optimized further, both in the move generation and the engine itself. For example, there are likely several areas where SIMD instructions are possible but are not picked up by the compiler. The minmax algorithm itself could also be optimized further, primarily through the use of a better ordering function. 
This ordering function is used to estimate which moves are good, and estimate their value first, making it easier to prune later branches, when these are found to be worse. For example, it is uneccessary to consider castling, if you have an opportunity to capture the opponent's queen. When generating moves for each piece, it would also be beneficial to add them to a binary heap instead of a vector, this would be faster since the vector is later sorted. The reason this is not yet implemented is because each move's estimated value is not only reliant on the move itself, but also the state of the board, which is not available through the move, making it more difficult to implement an ordering funciton for the moves. 

The underlying position evaluation function should also be tinkered with, there is no guarantee that the piece values or piece square value tables used now are ideal.

Finally, the engine should implement iterative deepening, so that it can be given a move-timer instead simply a static depth to search to as it is now. This would of course make it more enjoyable to play against, but also strengthen its endgames. A depth of four has been chosen purely to make it tolerable to play against in the early game, while the endgame is less computationally intensive and is therefore currently limited by the chosen depth.

The frontend has not been a priority of this project, but the primary weakness there is that the user is unable to under-promote. The loading graphics are also not centered, and are unreliable if the backend crashes.
