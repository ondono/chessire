use super::attacks::*;
use super::BitBoard;
use super::{generate_bishop_relevant_ocupancy, generate_rook_relevant_ocupancy, set_occupancy};
use rand::*;

#[allow(dead_code)]
fn generate_random_number() -> u64 {
    let mut rng = thread_rng();
    rng.gen_range(0..(u64::MAX & 0xFF00000000000000))
}

#[allow(dead_code)]
fn find_magic_number(sq: usize, num_bits: usize, bishop: bool) -> u64 {
    let mut occupancies = [BitBoard::new(0); 4096];
    let mut attacks = [BitBoard::new(0); 4096];

    let attack_mask = if bishop {
        generate_bishop_relevant_ocupancy(sq)
    } else {
        generate_rook_relevant_ocupancy(sq)
    };

    let oc_indexes: usize = 1 << num_bits;

    for index in 0..oc_indexes {
        occupancies[index] = set_occupancy(index, num_bits, &mut attack_mask.clone());
        attacks[index] = if bishop {
            bishop_attacks_on_the_fly(sq, occupancies[index])
        } else {
            rook_attacks_on_the_fly(sq, occupancies[index])
        };
    }

    // test magic numbers
    //
    for _random_count in 0..100000000 {
        let candidate = generate_random_number();

        //println!("Testing candidate:{}", candidate);
        if BitBoard::new(candidate).popcount() < 6 {
            //println!("Candidate had too low popcount");
            continue;
        }

        let c = candidate as u128;
        let a = attack_mask.get() as u128;

        if ((c * a) & 0xFF00000000000000) != 0 {
            continue;
        }

        // init used attacks
        let mut used_attacks = [BitBoard::new(0); 4096];

        let mut fail = false;

        // test magic index loop
        for index in 0..oc_indexes {
            if fail {
                break;
            }
            // init magic index
            let magic_index = ((occupancies[index].get().checked_mul(candidate).unwrap())
                >> (64 - num_bits)) as usize;

            // if magic index works
            if used_attacks[magic_index].get() == 0 {
                // initialize used attacks
                used_attacks[magic_index] = attacks[index];
            } else if used_attacks[magic_index].get() != attacks[index].get() {
                // magic index doesn't work
                fail = true;
            }
            if !fail {
                return candidate;
            }
        }
    }
    0
}
