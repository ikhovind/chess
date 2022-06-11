use crate::game::board_consts::{FILE_A, FILE_H, RANK_4, RANK_8};
use crate::print_u64_bitboard;

mod board_consts;

//[black, white]
pub struct Board {
    pub(crate) pawns: [u64; 2],
    pub(crate) knights: [u64; 2],
    pub(crate) bishops: [u64; 2],
    pub(crate) rooks: [u64; 2],
    pub(crate) queens: [u64; 2],
    pub(crate) kings: [u64; 2],
    black_pieces: u64,
    empty: u64,
}

impl Board {
    pub fn new(_pawns :[u64; 2], _knights :[u64; 2], _bishops :[u64; 2], _rooks :[u64; 2], _queens :[u64; 2], _kings: [u64; 2]) -> Board {
        let empty = !_pawns[0] & !_pawns[1] & !_knights[0] & !_knights[1] & !_bishops[0] & !_bishops[1] & !_rooks[0] & !_rooks[1] & !_queens[0] & !_queens[1] & !_kings[0] & !_kings[1];
        let black = _pawns[0]  | _knights[0] | _bishops[0] | _rooks[0]  | _queens[0] | _kings[0];
        return Board {pawns: _pawns, knights: _knights, bishops: _bishops, rooks: _rooks, queens: _queens, kings: _kings, black_pieces: black, empty}
    }

    pub fn possible_pw(&mut self, history: String) -> String {
        let mut list = String::from("");

        let mut pawn_moves = (self.pawns[1] << 9) & (self.black_pieces) & (!RANK_8) & (!FILE_A); // capture left

        for i in 0..64 {
            if ((pawn_moves >> i) & 1) == 1 {
                print!("{} {} {} {}", (i / 8), (i % 8),(i / 8 + 1), (i % 8 + 1));
                list.push_str(&*((i / 8) + (i % 8) + (i / 8 + 1) + (i % 8 + 1)).to_string());
            }
        }

        pawn_moves = (self.pawns[1] << 7) & (self.black_pieces) & (!RANK_8) & (!FILE_H); // capture left
        print_u64_bitboard(self.pawns[1] | self.pawns[0]);
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                print!("{} {} {} {}", (i / 8), (i % 8 + 2), (i / 8 + 1), (i % 8 + 1));
                list.push_str(&*((i/8)+(i%8 + 1)+(i/8)+(i%8)).to_string());
            }
        }
        pawn_moves=(self.pawns[1] << 8)&self.empty&!RANK_8;//move 1 forward
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push_str(&*((i/8+1)+(i%8)+(i/8)+(i%8)).to_string());
            }
        }
        pawn_moves=(self.pawns[1] << 16)&self.empty&(self.empty << 8)&RANK_4;//move 2 forward
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push_str(&*((i/8+2)+(i%8)+(i/8)+(i%8)).to_string());
            }
        }
        //y1,y2,Promotion Type,"P"
        pawn_moves=(self.pawns[1] << 7)&self.black_pieces&RANK_8&!FILE_A;//pawn promotion by capture right
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {list.push_str(&*format!("{}{}{}{}{}{}{}{}", (i % 8 - 1) + (i % 8), "QP", (i % 8 - 1) + (i % 8), "RP", (i % 8 - 1) + (i % 8), "BP", (i % 8 - 1) + (i % 8), "NP"));}
        }
        pawn_moves=(self.pawns[1] << 9)&self.black_pieces&RANK_8&!FILE_H;//pawn promotion by capture left
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {list.push_str(&*format!("{}{}{}{}{}{}{}{}", (i%8+1)+(i%8),"QP",(i%8+1)+(i%8),"RP",(i%8+1)+(i%8),"BP",(i%8+1)+(i%8),"NP"))}
        }
        pawn_moves=(self.pawns[1] << 8)&self.empty&RANK_8;//pawn promotion by move 1 forward
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {list.push_str(&*format!("{}{}{}{}{}{}{}{}",(i%8)+(i%8),"QP",(i%8)+(i%8),"RP",(i%8)+(i%8),"BP",(i%8)+(i%8),"NP"));}
        }
        return list;
    }
}
