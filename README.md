# Shellfish - [ikhovind.no](https://ikhovind.no)

This is a multi-threaded bitboard chess engine using a minmax algorithm with alpha beta pruning written entirely in Rust. 

The engine and a simple JavaScript frontend are deployed on Google Cloud, hosted on https://ikhovind.no

## Resources

Resources that have been especially useful during development have been:

   * [the Chess Programming Wiki](https://www.chessprogramming.org/)
   * [Sebastian Lague's video on Chess Programming](https://www.youtube.com/watch?v=U4ogK0MIzqk) 
   * [The YouTube channel Logic Crazy Chess](https://www.youtube.com/logiccrazyguide)

## Dependencies

The frontend largely relies on [Chessboard.js](https://chessboardjs.com/) in order to draw the board and pieces. 
Showing the user which moves are available is done through [Chess.js](https://github.com/jhlywa/chess.js), but all calculations of moves made by the AI is only made on the backend.

The backend relies on a number of non-chess related dependencies, all of which can be seen in the Cargo.toml

## TODO
 * Opening book  
