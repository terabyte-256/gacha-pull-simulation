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
                let mut rng = thread_rng();
                let random_value: f64 = rng.gen();
                let mut item_obtained = false;

                // Check for 5-star item first (character or weapon)
                let five_star_obtained = if pull_for_character || limited_successes < 7 {
                    // Character banner or still pulling for characters
                    let curr_five_star_chance_with_pity = curr_five_star_chance +
                        curr_soft_pity_increment * std::cmp::max(curr_char_pity - curr_character_soft_pity, 0) as f64;

                    if random_value <= curr_five_star_chance_with_pity || curr_char_pity + 1 == curr_character_pity {
                        // 5-star obtained, determine if it's limited
                        if curr_character_guaranteed || rng.gen::<f64>() <= curr_limited_character_chance {
                            five_char_successes += 1;
                            limited_successes += 1;
                            curr_character_guaranteed = false;
                            item_obtained = true;

                            if pull_for_character {
                                break 'pull_loop; // Stop if we're targeting character and got it
                            }
                        } else {
                            curr_character_guaranteed = true;
                            item_obtained = true;
                        }
                        curr_char_pity = 0;
                        true
                    } else {
                        curr_char_pity += 1;
                        false
                    }
                } else {
                    // Weapon banner or targeting weapon
                    let curr_five_star_chance_with_pity = curr_five_star_weapon_chance +
                        curr_soft_pity_increment * std::cmp::max(curr_weapon_pity - curr_weapon_soft_pity, 0) as f64;

                    if random_value <= curr_five_star_chance_with_pity || curr_weapon_pity + 1 == curr_weapon_pity_value {
                        // 5-star weapon obtained, determine if it's limited
                        if curr_weapon_guaranteed || rng.gen::<f64>() <= curr_limited_weapon_chance {
                            weapon_successes += 1;
                            curr_weapon_guaranteed = false;
                            item_obtained = true;

                            if !pull_for_character {
                                break 'pull_loop; // Stop if we're targeting weapon and got it
                            }
                        } else {
                            curr_weapon_guaranteed = true;
                            item_obtained = true;
                        }
                        curr_weapon_pity = 0;
                        true
                    } else {
                        curr_weapon_pity += 1;
                        false
                    }
                };

                // Only check for 4-star if we didn't get a 5-star
                if !five_star_obtained {
                    // Check for 4-star
                    if random_value <= curr_four_star_chance || curr_four_star_pity + 1 >= four_star_pity {
                        four_char_success += 1;
                        curr_four_star_pity = 0;
                        item_obtained = true;
                    } else {
                        curr_four_star_pity += 1;
                    }
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
