# Shellfish

This is a simple, but working chess engine developed entirely in Rust. 

The engine and a simple JavaScript frontend are deployed on Google Cloud, hosted on https://ikhovind.no

For simplicity's sake, neither the frontend nor backend implement any established chess communication protocol,
instead communicating through a websocket connection.



## Dependencies

The frontend largely relies on [Chessboard.js](https://chessboardjs.com/) in order to draw the board and pieces. 
Showing the user which moves are available is done through [Chess.js](https://github.com/jhlywa/chess.js), but all calculations of moves made by the AI is only made on the backend.

The backend relies on a number of non-chess related dependencies, all of which can be seen in the Cargo.toml

## TODO
 * Opening book
 * Various optimizations
    * Keep running sum of piece values
    * Sort checks earlier in the estimated move values
  
