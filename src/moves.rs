use crate::board::*;
use crate::piece::*;
use crate::piece::*;
use std::fmt;

// First, let's ease the algebra
// let's define the names from white's perspective
// rank 1 = BOTTOM
// rank 8 = TOP
// file A = RIGHT
// file H = LEFT

const VERTICAL_MOVE: usize = 16;
const HORIZONTAL_MOVE: usize = 1;

#[inline(always)]
pub fn next_up(index: usize) -> Option<usize> {
    let x = index + VERTICAL_MOVE;
    validated_position(x)
}

#[inline(always)]
pub fn next_down(index: usize) -> Option<usize> {
    let x = index.checked_sub(VERTICAL_MOVE).unwrap_or(0xff);
    validated_position(x)
}

#[inline(always)]
pub fn next_right(index: usize) -> Option<usize> {
    let x = index.checked_sub(HORIZONTAL_MOVE).unwrap_or(0xff);
    validated_position(x)
}

#[inline(always)]
pub fn next_left(index: usize) -> Option<usize> {
    let x = index + HORIZONTAL_MOVE;
    validated_position(x)
}

#[inline(always)]
pub fn next_up_right(index: usize) -> Option<usize> {
    // we can skip the sub check, because VERTICAL - HORIZONTAL > 0
    let x = index + VERTICAL_MOVE - HORIZONTAL_MOVE;
    validated_position(x)
}

#[inline(always)]
pub fn next_up_left(index: usize) -> Option<usize> {
    let x = index + VERTICAL_MOVE + HORIZONTAL_MOVE;
    validated_position(x)
}
#[inline(always)]
pub fn next_down_right(index: usize) -> Option<usize> {
    let x = index
        .checked_sub(HORIZONTAL_MOVE + VERTICAL_MOVE)
        .unwrap_or(0xff);
    validated_position(x)
}

#[inline(always)]
pub fn next_down_left(index: usize) -> Option<usize> {
    let x = index
        .checked_sub(VERTICAL_MOVE - HORIZONTAL_MOVE)
        .unwrap_or(0xff);
    validated_position(x)
}
