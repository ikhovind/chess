pub use crate::game::board_consts::{FILE_A, FILE_H, RANK_4, RANK_8, FILE_G, FILE_C};
use crate::{mv, print_u64_bitboard};
use crate::mv::{BISHOP, KNIGHT, Move, QUEEN, ROOK};

pub(crate) mod board_consts;

//[black, white]
pub struct Board {
    pub(crate) pawns: [u64; 2],
    pub(crate) knights: [u64; 2],
    pub(crate) bishops: [u64; 2],
    pub(crate) rooks: [u64; 2],
    pub(crate) queens: [u64; 2],
    pub(crate) kings: [u64; 2],
    black_pieces: u64,
    white_pieces: u64,
    empty: u64,
}

impl Board {
    pub fn from_fen(fen: String) -> Board {
        let mut _pawns = [0,0];
        let mut _bishops = [0,0];
        let mut _rooks = [0,0];
        let mut _knights = [0,0];
        let mut _queens = [0,0];
        let mut _kings = [0,0];

        let mut column : u32 = 0;
        let mut row = 7;
        let mut res = 0;
        let mut white = 1;
        for i in fen.chars() {
            if i.is_alphabetic() {
                res = 2_u64.pow((column + row * 8) as u32);
                //print_u64_bitboard(res);
            }
            if !i.is_uppercase() {
                white = 0;
            }
            else {
                white = 1
            }
            match i.to_ascii_lowercase() {
                'p' => {
                    column += 1;
                    _pawns[white] |= res;
                }
                'r' => {
                    column += 1;
                    _rooks[white] |= res;
                }
                'n' => {
                    column += 1;
                    _knights[white] |= res;
                }
                'q' => {
                    column += 1;
                    _queens[white] |= res;
                }
                'k' => {
                    column += 1;
                    _kings[white] |= res;
                }
                'b' => {
                    column += 1;
                    _bishops[white] |= res;
                }
                '/' => {
                    row-=1;
                    column = 0;
                }
                ' ' => {
                    break;
                }
                i if i.is_numeric() => {
                    column += i.to_digit(10).unwrap();
                }
                _ => {}
            }
        }
        let empty = !_pawns[0] & !_pawns[1] & !_knights[0] & !_knights[1] & !_bishops[0] & !_bishops[1] & !_rooks[0] & !_rooks[1] & !_queens[0] & !_queens[1] & !_kings[0] & !_kings[1];
        let black = _pawns[0]  | _knights[0] | _bishops[0] | _rooks[0]  | _queens[0] | _kings[0];
        return Board {pawns: _pawns, knights: _knights, bishops: _bishops, rooks: _rooks, queens: _queens, kings: _kings, black_pieces: black, white_pieces: (!(empty | black)), empty}
    }

    pub fn possible_p(&mut self, last_move: Move, white: usize) -> Vec<Move> {
        let mut list: Vec<Move> = Vec::new();
        let opposing_pieces = if white == 1 { self.black_pieces } else { self.white_pieces };
        let mut pawn_moves = (self.pawns[white] << 9) & (opposing_pieces) & (!RANK_8) & (!FILE_A); // capture left

        for i in 0..64 {
            if ((pawn_moves >> i) & 1) == 1 {
                list.push(Move::new_move(1,1, false));
                //list.push_str(&*((i / 8) + (i % 8) + (i / 8 + 1) + (i % 8 + 1)).to_string());
            }
        }

        pawn_moves = (self.pawns[white] << 7) & (opposing_pieces) & (!RANK_8) & (!FILE_H); // capture right
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push(Move::new_move(1,1, false));
                //list.push_str(&*((i/8)+(i%8 + 1)+(i/8)+(i%8)).to_string());
            }
        }
        pawn_moves=(self.pawns[white] << 8)&self.empty&!RANK_8;//move 1 forward
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push(Move::new_move(1,1, false));
                //list.push_str(&*((i/8+1)+(i%8)+(i/8)+(i%8)).to_string());
            }
        }
        pawn_moves=((self.pawns[white] << 16) & (self.empty & (self.empty << 8))) & RANK_4;//move 2 forward
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push(Move::new_move(1,1, false));
                //list.push_str(&*((i/8+2)+(i%8)+(i/8)+(i%8)).to_string());
            }
        }
        //y1,y2,Promotion Type,"P"
        pawn_moves=(self.pawns[white] << 7)&opposing_pieces&RANK_8&!FILE_A;//pawn promotion by capture left
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push(Move::new_move(1,1, false));
                //list.push_str(&*format!("{}{}{}{}{}{}{}{}", (i % 8 - 1) + (i % 8), "QP", (i % 8 - 1) + (i % 8), "RP", (i % 8 - 1) + (i % 8), "BP", (i % 8 - 1) + (i % 8), "NP"));
            }
        }
        pawn_moves=(self.pawns[white] << 9)&opposing_pieces&RANK_8&!FILE_H;//pawn promotion by capture right
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push(Move::new_promotion(1,1, true, QUEEN));
                list.push(Move::new_promotion(1,1, true, ROOK));
                list.push(Move::new_promotion(1,1, true, BISHOP));
                list.push(Move::new_promotion(1,1, true, KNIGHT));
                //list.push_str(&*format!("{}{}{}{}{}{}{}{}", (i%8+1)+(i%8),"QP",(i%8+1)+(i%8),"RP",(i%8+1)+(i%8),"BP",(i%8+1)+(i%8),"NP"))
            }
        }
        pawn_moves=(self.pawns[white] << 8)&self.empty&RANK_8;//pawn promotion by move 1 forward
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push(Move::new_promotion(1,1, false, QUEEN));
                list.push(Move::new_promotion(1,1, false, ROOK));
                list.push(Move::new_promotion(1,1, false, BISHOP));
                list.push(Move::new_promotion(1,1, false, KNIGHT));
                //list.push_str(&*format!("{}{}{}{}{}{}{}{}",(i%8)+(i%8),"QP",(i%8)+(i%8),"RP",(i%8)+(i%8),"BP",(i%8)+(i%8),"NP"));
            }
        }
        pawn_moves = ((self.pawns[white] << 9) & (opposing_pieces << 8) & (!RANK_8) & (!FILE_H)) & if Move::last_move_was_double_push(last_move) { 2_u64.pow(last_move.get_to_square() as u32) << 8} else { 0 };  // capture right
        for i in 0..64 {
            if ((pawn_moves >> i) & 1) == 1 {
                list.push(Move::new_move(1,1, false));
                //list.push_str(&*((i / 8) + (i % 8) + (i / 8 + 1) + (i % 8 + 1)).to_string());
            }
        }


        pawn_moves = ((self.pawns[white] << 7) & (opposing_pieces << 8) & (!RANK_8) & (!FILE_A)) & if Move::last_move_was_double_push(last_move) { 2_u64.pow(last_move.get_to_square() as u32) << 8} else { 0 }; // capture left
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push(Move::new_move(1,1, false));
                //list.push_str(&*((i/8)+(i%8 + 1)+(i/8)+(i%8)).to_string());
            }
        }
        return list;
    }
}
