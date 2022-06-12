pub use crate::game::board_consts::{FILE_MASKS, DIAGONAL_MASKS, ANTI_DIAGONAL_MASKS};
use crate::{mv, print_u64_bitboard};
use crate::game::board_consts::RANK_MASKS;
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
    white_long_c: bool,
    white_short_c: bool,
    black_long_c: bool,
    black_short_c: bool,
    watched_squares_white: u64,
    watched_squares_black: u64,
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
        let mut b = Board {
            pawns: _pawns,
            knights: _knights,
            bishops: _bishops,
            rooks: _rooks,
            queens: _queens,
            kings: _kings,
            black_pieces: black,
            white_pieces: (!(empty | black)),
            empty,
            white_long_c: true,
            white_short_c: true,
            black_long_c: true,
            black_short_c: true,
            watched_squares_black: 0,
            watched_squares_white: 0,
        };
        b.watched_squares_black = b.watched(false);
        b.watched_squares_white = b.watched(true);
        return b;
    }

    pub fn possible_p(&mut self, last_move: Move, white: usize) -> Vec<Move> {
        let mut list: Vec<Move> = Vec::new();
        let opposing_pieces = if white == 1 { self.black_pieces } else { self.white_pieces };
        let mut pawn_moves = (self.pawns[white] << 9) & (opposing_pieces) & (!RANK_MASKS[7]) & (!FILE_MASKS[0]); // capture right

        for i in 0..64 {
            if ((pawn_moves >> i) & 1) == 1 {
                list.push(Move::new_move(i - 9,i, true));
            }
        }

        pawn_moves = (self.pawns[white] << 7) & (opposing_pieces) & (!RANK_MASKS[7]) & (!FILE_MASKS[7]); // capture left
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push(Move::new_move(i-7,i, true));
            }
        }
        pawn_moves=(self.pawns[white] << 8)&self.empty&!RANK_MASKS[7];//move 1 forward
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push(Move::new_move(i-8,i, false));
            }
        }
        pawn_moves=((self.pawns[white] << 16) & (self.empty & (self.empty << 8))) & RANK_MASKS[3];//move 2 forward
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push(Move::new_move(i - 16,i, false));
            }
        }
        //y1,y2,Promotion Type,"P"
        pawn_moves=(self.pawns[white] << 7)&opposing_pieces&RANK_MASKS[7]&!FILE_MASKS[0];//pawn promotion by capture left
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push(Move::new_promotion(i - 7,i, true, QUEEN));
                list.push(Move::new_promotion(i - 7,i, true, ROOK));
                list.push(Move::new_promotion(i - 7,i, true, BISHOP));
                list.push(Move::new_promotion(i - 7,i, true, KNIGHT));
            }
        }

        pawn_moves=(self.pawns[white] << 9)&opposing_pieces&RANK_MASKS[7]&!FILE_MASKS[7];//pawn promotion by capture right
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push(Move::new_promotion(i - 9,i, true, QUEEN));
                list.push(Move::new_promotion(i - 9,i, true, ROOK));
                list.push(Move::new_promotion(i - 9,i, true, BISHOP));
                list.push(Move::new_promotion(i - 9,i, true, KNIGHT));
            }
        }

        pawn_moves=(self.pawns[white] << 8)&self.empty&RANK_MASKS[7];//pawn promotion by move 1 forward
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push(Move::new_promotion(i - 8,i, false, QUEEN));
                list.push(Move::new_promotion(i - 8,i, false, ROOK));
                list.push(Move::new_promotion(i - 8,i, false, BISHOP));
                list.push(Move::new_promotion(i - 8,i, false, KNIGHT));
            }
        }
        // en passant
        pawn_moves = ((self.pawns[white] << 9) & (opposing_pieces << 8) & (!RANK_MASKS[7]) & (!FILE_MASKS[7])) & if Move::last_move_was_double_push(last_move) { 2_u64.pow(last_move.get_to_square() as u32) << 8} else { 0 };  // capture right
        for i in 0..64 {
            if ((pawn_moves >> i) & 1) == 1 {
                list.push(Move::new_ep(i - 9, i));
            }
        }

        pawn_moves = ((self.pawns[white] << 7) & (opposing_pieces << 8) & (!RANK_MASKS[7]) & (!FILE_MASKS[0])) & if Move::last_move_was_double_push(last_move) { 2_u64.pow(last_move.get_to_square() as u32) << 8} else { 0 }; // capture left
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push(Move::new_ep(i - 7, i));
            }
        }
        return list;
    }

    pub fn possible_b(&self, white: bool) -> Vec<Move> {
        let mut opposing_pieces: u64 = self.white_pieces;
        let mut own_pieces = self.black_pieces;
        let mut index = 0;
        if white {
            opposing_pieces = self.black_pieces;
            own_pieces = self.white_pieces;
            index = 1;
        }
        let mut list: Vec<Move> = Vec::new();
        let bishops = self.bishops[index];
        for i in 0u8..64u8 {
            if 2_u64.pow(i as u32) & bishops != 0 {
                let moves = self.d_and_anti_d_moves(i as usize, white) & !(own_pieces);
                for i2 in 0u8..64u8 {
                    if 2u64.pow(i2 as u32) & moves != 0 {
                        list.push(
                            Move::new_move(i, i2, opposing_pieces & 2_u64.pow(i2 as u32) != 0)
                        );
                    }
                }
            }
        }
        return list;
    }

    pub fn possible_r(&self, white: bool) -> Vec<Move> {
        let mut opposing_pieces: u64 = self.white_pieces;
        let mut own_pieces = self.black_pieces;
        let mut index = 0;
        if white {
            opposing_pieces = self.black_pieces;
            own_pieces = self.white_pieces;
            index = 1;
        }
        let mut list: Vec<Move> = Vec::new();
        let rooks = self.rooks[index];
        for i in 0u8..64u8 {
            if 2_u64.pow(i as u32) & rooks != 0 {
                let moves = self.h_and_vmoves(i as usize, white) & !(own_pieces);
                for i2 in 0u8..64u8 {
                    if 2u64.pow(i2 as u32) & moves != 0 {
                        list.push(
                            Move::new_move(i, i2, opposing_pieces & 2_u64.pow(i2 as u32) != 0)
                        );
                    }
                }
            }
        }
        return list;
    }

    pub fn possible_q(&self, white: bool) -> Vec<Move> {
        let mut opposing_pieces: u64 = self.white_pieces;
        let mut own_pieces = self.black_pieces;
        let mut index = 0;
        if white {
            opposing_pieces = self.black_pieces;
            own_pieces = self.white_pieces;
            index = 1;
        }
        let mut list: Vec<Move> = Vec::new();
        let queens = self.queens[index];
        for i in 0u8..64u8 {
            if 2_u64.pow(i as u32) & queens != 0 {
                let moves = (self.d_and_anti_d_moves(i as usize, white) | self.h_and_vmoves(i as usize, white)) & !(own_pieces);
                for i2 in 0u8..64u8 {
                    if 2u64.pow(i2 as u32) & moves != 0 {
                        list.push(
                            Move::new_move(i, i2, opposing_pieces & 2_u64.pow(i2 as u32) != 0)
                        );
                    }
                }
            }
        }
        return list;
    }

    pub fn possible_k(&self, white: bool) -> Vec<Move> {
        let mut opposing_pieces: u64 = self.white_pieces;
        let mut opponent_watching: u64 = self.watched_squares_white;
        let mut own_pieces = self.black_pieces;
        let mut index = 0;
        let mut short_castle= self.black_short_c;
        let mut long_castle = self.black_long_c;
        let mut short_castle_sq = vec![
            1 << 60,
            1 << 61,
            1 << 62,
        ];
        let mut long_castle_sq = vec![
            1 << 58,
            1 << 59,
            1 << 60,
        ];
        if white {
            opposing_pieces = self.black_pieces;
            opponent_watching = self.watched_squares_black;
            own_pieces = self.white_pieces;
            index = 1;
            short_castle = self.white_short_c;
            long_castle = self.white_long_c;

            long_castle_sq = vec![
                1 << 1,
                1 << 2,
                1 << 3,
                1 << 4
            ];
            short_castle_sq = vec![
                1 << 4,
                1 << 5,
                1 << 6,
            ];
        }
        let mut list: Vec<Move> = Vec::new();
        let kings = self.kings[index];

        for i in 0u8..64u8 {
            if 2_u64.pow(i as u32) & kings != 0 {
                let king_loc = 1 << i;
                let king_clip_file_h = king_loc & !FILE_MASKS[7];
                let king_clip_file_a = king_loc & !FILE_MASKS[0];

                /* remember the representation of the board in relation to the bitindex
                    when looking at these shifts.... */
                let spot_1 = king_clip_file_h << 7;
                let spot_2 = king_loc << 8;
                let spot_3 = king_clip_file_h << 9;
                let spot_4 = king_clip_file_h << 1;

                let spot_5 = king_clip_file_a >> 7;
                let spot_6 = king_loc >> 8;
                let spot_7 = king_clip_file_a >> 9;
                let spot_8 = king_clip_file_a >> 1;

                let moves = (spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 |
                    spot_7 | spot_8) & !own_pieces & !opponent_watching;

                if long_castle && !long_castle_sq.iter().any(|&x| (x & ((self.white_pieces - self.kings[1]) | (self.black_pieces - self.kings[0])) != 0)) {
                    list.push(Move::new_castle(if white { 4 } else { 60 }, if white { 2 } else { 58 }));
                }

                if short_castle && !short_castle_sq.iter().any(|&x| (x & ((self.white_pieces - self.kings[1]) | (self.black_pieces - self.kings[0])) != 0)) {
                    list.push(Move::new_castle(if white { 4 } else { 60 }, if white {6} else { 62 }));
                }

                /* compute only the places where the king can move and attack. The caller
                    will interpret this as a white or black king. */
                for i2 in 0u8..64u8 {
                    if 2u64.pow(i2 as u32) & moves != 0 {
                        list.push(
                            Move::new_move(i, i2, opposing_pieces & 2_u64.pow(i2 as u32) != 0)
                        );
                    }
                }
            }
        }
        return list;
    }

    pub fn possible_n(&self, white: bool) -> Vec<Move> {
        let mut opposing_pieces: u64 = self.white_pieces;
        let mut own_pieces = self.black_pieces;
        let mut index = 0;
        if white {
            opposing_pieces = self.black_pieces;
            own_pieces = self.white_pieces;
            index = 1;
        }
        let mut list: Vec<Move> = Vec::new();

        let knights = self.knights[index];
        for i in 0u8..64u8 {
            if 2_u64.pow(i as u32) & knights != 0 {
                let spot_1_clip = !(FILE_MASKS[0] & FILE_MASKS[1]);
                let spot_2_clip = !FILE_MASKS[0];
                let spot_3_clip = !FILE_MASKS[7];
                let spot_4_clip = !(FILE_MASKS[7] & FILE_MASKS[6]);

                let spot_5_clip = !(FILE_MASKS[7] & FILE_MASKS[6]);
                let spot_6_clip = !FILE_MASKS[7];
                let spot_7_clip = !FILE_MASKS[0];
                let spot_8_clip = !(FILE_MASKS[0] & FILE_MASKS[1]);

                /* The clipping masks we just created will be used to ensure that no
            under or overflow positions are computed when calculating the
            possible moves of the knight in certain files. */

                let spot_1 = ((1 << i)  & spot_1_clip) << 6;
                let spot_2 = ((1 << i) & spot_2_clip) << 15;
                let spot_3 = ((1 << i) & spot_3_clip) << 17;
                let spot_4 = ((1 << i)  & spot_4_clip) << 10;

                let spot_5 = ((1 << i)  & spot_5_clip) >> 6;
                let spot_6 = ((1 << i) & spot_6_clip) >> 15;
                let spot_7 = ((1 << i) & spot_7_clip) >> 17;
                let spot_8 = ((1 << i) & spot_8_clip) >> 10;

                let moves =
                    (spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 | spot_7 | spot_8)
                        & !own_pieces;
                for i2 in 0u8..64u8 {
                    if 2u64.pow(i2 as u32) & moves != 0 {
                        list.push(
                            Move::new_move(
                                i,
                                i2,
                                opposing_pieces & 2_u64.pow(i2 as u32) != 0
                            )
                        );
                    }
                }
            }
        }

        /* compute only the places where the knight can move and attack. The
            caller will determine if this is a white or black night. */
        return list;
    }

    pub fn watched_by_k(&self, white: bool) -> u64 {
        let mut index = 0;
        if white {
            index = 1;
        }
        let mut moves = 0;
        let kings = self.kings[index];

        for i in 0u8..64u8 {
            if 2_u64.pow(i as u32) & kings != 0 {
                let king_loc = 1 << i;
                let king_clip_file_h = king_loc & !FILE_MASKS[7];
                let king_clip_file_a = king_loc & !FILE_MASKS[0];

                /* remember the representation of the board in relation to the bitindex
                    when looking at these shifts.... */
                let spot_1 = king_clip_file_h << 7;
                let spot_2 = king_loc << 8;
                let spot_3 = king_clip_file_h << 9;
                let spot_4 = king_clip_file_h << 1;

                let spot_5 = king_clip_file_a >> 7;
                let spot_6 = king_loc >> 8;
                let spot_7 = king_clip_file_a >> 9;
                let spot_8 = king_clip_file_a >> 1;

                moves = moves | spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 |
                    spot_7 | spot_8;
            }
        }
        return moves;
    }

    pub fn watched_by_n(&self, white:bool) -> u64 {
        let mut index = 0;
        if white {
            index = 1;
        }
        let mut moves = 0;
        let knights = self.knights[index];
        for i in 0u8..64u8 {
            if (1 << i) & knights != 0 {
                let spot_1_clip = (!FILE_MASKS[0] & !FILE_MASKS[1]);
                let spot_2_clip = !FILE_MASKS[0];
                let spot_3_clip = !FILE_MASKS[7];
                let spot_4_clip = (!FILE_MASKS[7] & !FILE_MASKS[6]);

                let spot_5_clip = (!FILE_MASKS[7] & !FILE_MASKS[6]);
                let spot_6_clip = !FILE_MASKS[7];
                let spot_7_clip = !FILE_MASKS[0];
                let spot_8_clip = (!FILE_MASKS[0] & !FILE_MASKS[1]);

                /* The clipping masks we just created will be used to ensure that no
            under or overflow positions are computed when calculating the
            possible moves of the knight in certain files. */

                let spot_1 = ((1 << i) & spot_1_clip) << 6;
                let spot_2 = ((1 << i) & spot_2_clip) << 15;
                let spot_3 = ((1 << i) & spot_3_clip) << 17;
                let spot_4 = ((1 << i) & spot_4_clip) << 10;

                let spot_5 = ((1 << i) & spot_5_clip) >> 6;
                let spot_6 = ((1 << i) & spot_6_clip) >> 15;
                let spot_7 = ((1 << i) & spot_7_clip) >> 17;
                let spot_8 = ((1 << i) & spot_8_clip) >> 10;

                moves = moves | spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 | spot_7 | spot_8;
            }
        }
        return moves;
    }

    pub fn watched_by_p(&self, white: bool) -> u64 {
        let index = if white { 1 } else { 0 };
        let mut pawn_moves = (self.pawns[index] << 9) & (!RANK_MASKS[7]) & (!FILE_MASKS[0]); // capture right
        pawn_moves = pawn_moves | ((self.pawns[index] << 7) & (!RANK_MASKS[7]) & (!FILE_MASKS[7])); // capture left

        return pawn_moves;
    }

    pub fn watched_by_b(&self, white: bool) -> u64 {
        let mut index = 0;
        let mut moves = 0;
        if white {
            index = 1;
        }
        let bishops = self.bishops[index];
        for i in 0u8..64u8 {
            if 2_u64.pow(i as u32) & bishops != 0 {
                moves |= self.d_and_anti_d_moves(i as usize, white);
            }
        }
        return moves;
    }

    pub fn watched_by_r(&self, white: bool) -> u64 {
        let mut index = 0;
        let mut moves = 0;
        if white {
            index = 1;
        }
        let rooks = self.rooks[index];
        for i in 0u8..64u8 {
            if 2_u64.pow(i as u32) & rooks != 0 {
                moves |= self.h_and_vmoves(i as usize, white);
            }
        }
        return moves;
    }

    pub fn watched_by_q(&self, white: bool) -> u64 {
        let mut index = 0;
        let mut moves = 0;
        if white {
            index = 1;
        }
        let queens = self.queens[index];
        for i in 0u8..64u8 {
            if 2_u64.pow(i as u32) & queens != 0 {
                moves |= (self.d_and_anti_d_moves(i as usize, white) | self.h_and_vmoves(i as usize, white));
            }
        }
        return moves;
    }

    pub fn watched(&self, white: bool) -> u64 {
        return self.watched_by_b(white) | self.watched_by_k(white) | self.watched_by_n(white) | self.watched_by_q(white) | self.watched_by_r(white) | self.watched_by_p(white);
    }
    pub fn h_and_vmoves(&self, s: usize, white: bool) -> u64 {
        let binary_s:u64 = 1<<s;
        let king = if white { self.kings[0] } else { self.kings[1] };
        let possibilities_horizontal: u64 = (((self.white_pieces | self.black_pieces) - king) - 2 * binary_s) ^ (((self.white_pieces | self.black_pieces) - king).reverse_bits() - 2 * binary_s.reverse_bits()).reverse_bits();
        let possibilities_vertical: u64 = ((((self.white_pieces | self.black_pieces) - king) & FILE_MASKS[s % 8]) - (2 * binary_s)) ^ ((((self.white_pieces | self.black_pieces) - king)& FILE_MASKS[s % 8]).reverse_bits() - (2 * binary_s.reverse_bits())).reverse_bits();
        return (possibilities_horizontal & RANK_MASKS[s / 8]) | (possibilities_vertical & FILE_MASKS[s % 8]);
    }

    pub fn d_and_anti_d_moves(&self, s: usize, white: bool) -> u64 {
        let binary_s:u64 = 1 << s;
        let king = if white { self.kings[0] } else { self.kings[1] };
        let possibilities_diagonal: u64 = ((((self.white_pieces | self.black_pieces) - king)&DIAGONAL_MASKS[(s / 8) + (s % 8)]) - (2 * binary_s)) ^ ((((self.white_pieces | self.black_pieces) - king)&DIAGONAL_MASKS[(s / 8) + (s % 8)]).reverse_bits() - (2 * (binary_s).reverse_bits())).reverse_bits();
        let possibilities_anti_diagonal: u64 = ((((self.white_pieces | self.black_pieces) - king)&ANTI_DIAGONAL_MASKS[(s / 8) + 7 - (s % 8)]) - (2 * binary_s)) ^ ((((self.white_pieces | self.black_pieces) - king)&ANTI_DIAGONAL_MASKS[(s / 8) + 7 - (s % 8)]).reverse_bits() - (2 * (binary_s).reverse_bits())).reverse_bits();
        return (possibilities_diagonal &DIAGONAL_MASKS[(s / 8) + (s % 8)]) | (possibilities_anti_diagonal &ANTI_DIAGONAL_MASKS[(s / 8) + 7 - (s % 8)]);
    }
}
