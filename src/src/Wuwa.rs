pub mod Wuwa {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use rand::Rng;
    use rand::thread_rng;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref COMMON_FIVE_CHANCE: f64 = 0.008;
        static ref COMMON_FOUR_STAR_CHANCE: f64 = 0.06;
        static ref FOUR_STAR_PITY: i32 = 10;
        static ref FIVE_STAR_PITY: i32 = 80;
    }

    // number of pulls till one five star
    pub fn simulate_game(num_simulations: i32, pull_for_character: bool) -> Vec<(i32, i32, i32, i32, i32)> {
            let mut results = Vec::new();
            for _ in 0..num_simulations {
                let mut pulls = 0;
                let mut five_successes = 0;
                let mut four_successes = 0;
                let mut limited_four_successes = 0;
                let mut three_successes = 0;
                let mut curr_five_pity = 0;
                let mut curr_four_pity = 0;
                let mut four_guaranteed = false;


                loop {
                    pulls += 1;
                    let random_value: f64 = thread_rng().gen();
                    let five_star_chance = *COMMON_FIVE_STAR_CHANCE;
                    let four_star_chance = *COMMON_FOUR_STAR_CHANCE;
                    let five_pity = *FIVE_STAR_PITY;
                    let four_pity = *FOUR_STAR_PITY;

                    if curr_five_pity + 1 == five_pity || random_value <= five_star_chance {
                        five_successes += 1;
                        curr_five_pity = 0;
                        break;
                    } else {
                        if curr_four_pity + 1 == four_pity || random_value <= four_star_chance {
                            if four_guaranteed || thread_rng().gen::<f64>() <= 0.5 {
                                limited_four_successes += 1;
                                four_pity = 0;
                            } else {
                                four_guaranteed = true;
                                four_successes += 1;
                                four_pity = 0;
                        } else {
                            three_successes += 1;
                        }
                    curr_five_pity += 1;
                    curr_four_pity += 1;
                }
                results.push((pulls, five_successes, four_successes, limited_four_successes, three_successes));
            }
            results
        }
}
