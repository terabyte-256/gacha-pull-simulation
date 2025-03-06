pub mod hoyo {
    use rand::Rng;
    use rand::thread_rng;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref COMMON_FIVE_STAR_CHARACTER_CHANCE: f64 = 0.006;
        static ref COMMON_SOFT_PITY_INCREMENT: f64 = 0.062;
        static ref COMMON_CHARACTER_SOFT_PITY: i32 = 74;
        static ref COMMON_CHARACTER_PITY: i32 = 90;
        static ref COMMON_WEAPON_SOFT_PITY: i32 = 64;
        static ref COMMON_WEAPON_PITY: i32 = 80;
        static ref COMMON_FOUR_STAR_CHARACTER_CHANCE: f64 = 0.051;
        static ref COMMON_FOUR_STAR_PITY: i32 = 10;
    }

    pub struct GameData {
        five_star_weapon_chance: f64,
        limited_character_chance: f64,
        limited_weapon_chance: f64,
    }

    impl GameData {
        pub fn new(five_star_weapon_chance: f64, limited_character_chance: f64, limited_weapon_chance: f64) -> Self {
            GameData {
                five_star_weapon_chance,
                limited_character_chance,
                limited_weapon_chance,
            }
        }
    }

    pub fn h_simulate_game(game_data: &GameData, num_simulations: i32, pull_for_character: bool) -> Vec<(i32, i32, i32, i32, i32)> {
        let mut results = Vec::with_capacity(num_simulations as usize);

        for _ in 0..num_simulations {
            let mut pulls = 0;
            let mut five_char_successes = 0;
            let mut four_char_success = 0;
            let mut three_char_success = 0;
            let mut weapon_successes = 0;
            let mut limited_successes = 0;
            let mut curr_weapon_pity = 0;
            let mut curr_char_pity = 0;
            let mut curr_weapon_guaranteed = false;
            let mut curr_character_guaranteed = false;
            let mut curr_four_star_pity = 0;

            // Constants lookup once outside the loop
            let curr_five_star_chance = *COMMON_FIVE_STAR_CHARACTER_CHANCE;
            let curr_four_star_chance = *COMMON_FOUR_STAR_CHARACTER_CHANCE;
            let curr_five_star_weapon_chance = game_data.five_star_weapon_chance;
            let curr_soft_pity_increment = *COMMON_SOFT_PITY_INCREMENT;
            let curr_character_soft_pity = *COMMON_CHARACTER_SOFT_PITY;
            let curr_character_pity = *COMMON_CHARACTER_PITY;
            let four_star_pity = *COMMON_FOUR_STAR_PITY;
            let curr_weapon_soft_pity = *COMMON_WEAPON_SOFT_PITY;
            let curr_weapon_pity_value = *COMMON_WEAPON_PITY;
            let curr_limited_weapon_chance = game_data.limited_weapon_chance;
            let curr_limited_character_chance = game_data.limited_character_chance;

            'pull_loop: loop {
                pulls += 1;
                let mut five_star_obtained = false;
                let mut item_obtained = false;

                if pull_for_character || limited_successes < 7 {
                    // Character banner logic
                    curr_char_pity += 1;

                    // HARD PITY CHECK - Guaranteed 5-star at exactly 90 pulls
                    if curr_char_pity == curr_character_pity {
                        // We've reached hard pity - guaranteed 5-star
                        if curr_character_guaranteed || thread_rng().gen::<f64>() <= curr_limited_character_chance {
                            five_char_successes += 1;
                            limited_successes += 1;
                            curr_character_guaranteed = false;
                        } else {
                            curr_character_guaranteed = true;
                        }
                        curr_char_pity = 0;
                        five_star_obtained = true;
                        item_obtained = true;

                        if pull_for_character {
                            break 'pull_loop; // Stop if we're targeting character and got it
                        }
                    }
                    // SOFT PITY & REGULAR ROLL CHECK
                    else {
                        let mut chance = curr_five_star_chance;
                        // Apply soft pity if applicable
                        if curr_char_pity > curr_character_soft_pity {
                            chance += curr_soft_pity_increment * (curr_char_pity - curr_character_soft_pity) as f64;
                        }

                        // Check if we get a 5-star based on the probability
                        if thread_rng().gen::<f64>() <= chance {
                            // Got a 5-star before hard pity
                            if curr_character_guaranteed || thread_rng().gen::<f64>() <= curr_limited_character_chance {
                                five_char_successes += 1;
                                limited_successes += 1;
                                curr_character_guaranteed = false;
                            } else {
                                curr_character_guaranteed = true;
                            }
                            curr_char_pity = 0;
                            five_star_obtained = true;
                            item_obtained = true;

                            if pull_for_character {
                                break 'pull_loop; // Stop if we're targeting character and got it
                            }
                        }
                    }
                } else {
                    // Weapon banner logic
                    curr_weapon_pity += 1;

                    // HARD PITY CHECK - Guaranteed 5-star at exactly 80 pulls
                    if curr_weapon_pity == curr_weapon_pity_value {
                        // We've reached hard pity - guaranteed 5-star weapon
                        if curr_weapon_guaranteed || thread_rng().gen::<f64>() <= curr_limited_weapon_chance {
                            weapon_successes += 1;
                            curr_weapon_guaranteed = false;
                        } else {
                            curr_weapon_guaranteed = true;
                        }
                        curr_weapon_pity = 0;
                        five_star_obtained = true;
                        item_obtained = true;

                        if !pull_for_character {
                            break 'pull_loop; // Stop if we're targeting weapon and got it
                        }
                    }
                    // SOFT PITY & REGULAR ROLL CHECK
                    else {
                        let mut chance = curr_five_star_weapon_chance;
                        // Apply soft pity if applicable
                        if curr_weapon_pity > curr_weapon_soft_pity {
                            chance += curr_soft_pity_increment * (curr_weapon_pity - curr_weapon_soft_pity) as f64;
                        }

                        // Check if we get a 5-star based on the probability
                        if thread_rng().gen::<f64>() <= chance {
                            // Got a 5-star before hard pity
                            if curr_weapon_guaranteed || thread_rng().gen::<f64>() <= curr_limited_weapon_chance {
                                weapon_successes += 1;
                                curr_weapon_guaranteed = false;
                            } else {
                                curr_weapon_guaranteed = true;
                            }
                            curr_weapon_pity = 0;
                            five_star_obtained = true;
                            item_obtained = true;

                            if !pull_for_character {
                                break 'pull_loop; // Stop if we're targeting weapon and got it
                            }
                        }
                    }
                }

                // Handle 4-star pity if we didn't get a 5-star
                if !five_star_obtained {
                    curr_four_star_pity += 1;

                    // Check for guaranteed 4-star at pity
                    if curr_four_star_pity == four_star_pity {
                        four_char_success += 1;
                        curr_four_star_pity = 0;
                        item_obtained = true;
                    }
                    // Regular chance for 4-star
                    else if thread_rng().gen::<f64>() <= curr_four_star_chance {
                        four_char_success += 1;
                        curr_four_star_pity = 0;
                        item_obtained = true;
                    }
                } else {
                    // Reset 4-star pity when 5-star is obtained
                    curr_four_star_pity = 0;
                }

                // If neither 5-star nor 4-star, it's a 3-star
                if !item_obtained {
                    three_char_success += 1;
                }
            }

            results.push((pulls, limited_successes, weapon_successes, four_char_success, three_char_success));
        }

        results
    }
}
