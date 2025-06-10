use std::mem;

use crate::{BitMove, MAX_DEPTH};

static mut PRIMARY_KILLER_MOVES: [BitMove; MAX_DEPTH] = unsafe { mem::zeroed() };
static mut SECONDARY_KILLER_MOVES: [BitMove; MAX_DEPTH] = unsafe { mem::zeroed() };

pub(crate) struct KillerMoves;

impl KillerMoves {
    #[inline(always)]
    pub(crate) fn get_primary(ply: u16) -> Option<BitMove> {
        if ply < MAX_DEPTH as u16 {
            unsafe { Some(PRIMARY_KILLER_MOVES[ply as usize]) }
        } else {
            None
        }
    }

    #[inline(always)]
    pub(crate) fn get_secondary(ply: u16) -> Option<BitMove> {
        if ply < MAX_DEPTH as u16 {
            unsafe { Some(SECONDARY_KILLER_MOVES[ply as usize]) }
        } else {
            None
        }
    }

    #[inline(always)]
    pub(crate) fn update(bit_move: BitMove, ply: u16) {
        if ply < MAX_DEPTH as u16 {
            unsafe {
                SECONDARY_KILLER_MOVES[ply as usize] = PRIMARY_KILLER_MOVES[ply as usize];
                PRIMARY_KILLER_MOVES[ply as usize] = bit_move;
            }
        }
    }

    #[inline(always)]
    pub(crate) fn reset() {
        unsafe {
            PRIMARY_KILLER_MOVES = mem::zeroed();
            SECONDARY_KILLER_MOVES = mem::zeroed();
        }
    }
}
