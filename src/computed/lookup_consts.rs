use std::collections::HashMap;
use crate::consts::board_consts::FILE_MASKS;

pub const KNIGHT_MOVES: [u64; 64] = calculate_knight_moves();

const fn calculate_knight_moves() -> [u64; 64] {
    let spot_1_clip = !FILE_MASKS[0] & !FILE_MASKS[1];
    let spot_2_clip = !FILE_MASKS[0];
    let spot_3_clip = !FILE_MASKS[7];
    let spot_4_clip = !FILE_MASKS[7] & !FILE_MASKS[6];

    let spot_5_clip = !FILE_MASKS[7] & !FILE_MASKS[6];
    let spot_6_clip = !FILE_MASKS[7];
    let spot_7_clip = !FILE_MASKS[0];
    let spot_8_clip = !FILE_MASKS[0] & !FILE_MASKS[1];

    /* The clipping masks we just created will be used to ensure that no
under or overflow positions are computed when calculating the
possible moves of the knight in certain files. */
    let mut ans: [u64; 64] = [0; 64];
    let mut i = 0;
    while i < 64 {
        let spot_1 = ((1 << i) & spot_1_clip) << 6;
        let spot_2 = ((1 << i) & spot_2_clip) << 15;
        let spot_3 = ((1 << i) & spot_3_clip) << 17;
        let spot_4 = ((1 << i) & spot_4_clip) << 10;

        let spot_5 = ((1 << i) & spot_5_clip) >> 6;
        let spot_6 = ((1 << i) & spot_6_clip) >> 15;
        let spot_7 = ((1 << i) & spot_7_clip) >> 17;
        let spot_8 = ((1 << i) & spot_8_clip) >> 10;
        let moves =
            (spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 | spot_7 | spot_8);
        ans[i as usize] = moves;
        i += 1;
    }
    return ans;
}
