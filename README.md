# Shellfish - [ikhovind.no](https://ikhovind.no)

This is a multi-threaded bitboard chess engine using a minmax algorithm with alpha beta pruning written entirely in Rust. 

The engine and a simple JavaScript front-end are deployed on Google Cloud, hosted on https://ikhovind.no

## Resources

Resources that have been especially useful during development have been:

   * [The Chess Programming Wiki](https://www.chessprogramming.org/)
   * [Sebastian Lague's video on Chess Programming](https://www.youtube.com/watch?v=U4ogK0MIzqk) 
   * [The YouTube channel Logic Crazy Chess](https://www.youtube.com/logiccrazyguide)

## Dependencies

The front-end largely relies on [Chessboard.js](https://chessboardjs.com/) in order to draw the board and pieces. 
Showing the user which moves are available is done through [Chess.js](https://github.com/jhlywa/chess.js), but all calculations of moves made by the AI is only made on the back-end.

The back-end relies on a number of non-chess related dependencies, all of which can be seen in the Cargo.toml

## Improvements
There engine has many areas of improvements, here broken down into three main categories:

### Features

**Draws**

First of all, the engine does not take draws by repetition or the 50-move rule into account, meaning that it often draws winning endgames, or loses where it would be able to force repetition. For the sake of execution speed, at least three move repetition should be handled by Zobrist hashing, which could also be used to greatly improve the general execution speed, especially in endgames.

**Iterative Deepening**

To make the engine more enjoyable to play against, it should implement a move timer. This could be achieved through iterative deepening, where searches are performed subsequently, from a depth of 1 to infinity, until the search is halted by the timer. This would ensure a useful result without having to know the depth beforehand. 

### Optimization

**SIMD**

One of the benefits of using bitboards, is that the use of 64 bit integers allows for easier use of SIMD instructions. There are many areas where these instructions would be beneficial for the engine, but it has not been checked if they are implemented by the compiler, or if they need to be implemented manually.

**Improved Ordering Function**

The minmax algorithm itself could also be optimized further, primarily through the use of a better ordering function. This ordering function is used to estimate which moves are good, and estimate their value first, making it easier to prune later branches. When generating moves for each piece, it would also be beneficial to add them to a binary heap instead of a vector as these can be more easily joined into a sorted vector. 

### Front-end

The front-end has not been a priority of this project, but the primary weakness there is that the user is unable to under-promote. The loading graphics are also not centered, and are unreliable if the back-end crashes.
