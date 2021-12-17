// New design for the move generator
// This structure holds all precomputed data that needs to be initialized at startup
// Then this information can be used to compute moves faster
//
// Two move generators will be supported (for now)
// - Bitboard (Default): the bitboard generator uses Magic Numbers, is fast but requires "heavy" amounts of
// memory (2-3MB). This makes it unsuitable for some embedded devices
// - Board88: A move generator based on 0x88 Board. Less memory intensive but slower.
//
//
