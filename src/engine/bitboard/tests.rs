use super::*;

// TESTS
#[test]
fn test_rook() {
    let mut attack_tables = AttackTables::new();
    attack_tables.init();

    // basic generation, covers only 1 blocker
    for block in 0..64 {
        let blockers = BitBoard::new(block);
        for sq in 0..64 {
            // compute the on the fly attacks
            let onthefly = rook_attacks_on_the_fly(sq, blockers);
            // compute bitboard attacks
            let bbattacks = get_rook_attack(&attack_tables, sq, blockers);
            // compare the two
            assert_eq!(bbattacks, onthefly, "Testcase sq:{} block:{}", sq, block);
        }
    }
}

#[test]
fn test_bishop() {
    let mut attack_tables = AttackTables::new();
    attack_tables.init();

    // basic generation, covers only 1 blocker
    for block in 0..64 {
        let blockers = BitBoard::new(block);
        for sq in 0..64 {
            // compute on the fly attacks
            let onthefly = bishop_attacks_on_the_fly(sq, blockers);
            // compute bitboard attacks
            let bbattacks = get_bishop_attack(&attack_tables, sq, blockers);
            // compare the two
            assert_eq!(bbattacks, onthefly, "Testcase sq:{} block:{}", sq, block);
        }
    }
}
