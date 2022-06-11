struct Move {
    //smallest 6 bits are to square, bit 7 is promotion, bit 8 is castle
    from: u8,
    // bit 7 and 8 are type of promotion / type of castle
    to: u8
}

impl Move {
    fn new_move(from: u8, to: u8, is_capture: bool) -> Move {
        return Move {from, to}
    }

    fn new_promotion(from: u8, to: u8, is_capture: bool, promote_to: u8) -> Move {
        return Move {from, to}
    }

    fn new_ep(from: u8, to:u8) {

    }

    fn new_castle(from: u8, to:u8) {

    }

}