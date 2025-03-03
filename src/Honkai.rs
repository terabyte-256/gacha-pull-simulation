pub mod honkai {
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
        let mut results = Vec::new();
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

            loop {
                pulls += 1;
                let random_value: f64 = thread_rng().gen();
                let curr_five_star_chance = *COMMON_FIVE_STAR_CHARACTER_CHANCE;
                let curr_four_star_chance = *COMMON_FOUR_STAR_CHARACTER_CHANCE;
                let curr_five_star_weapon_chance = game_data.five_star_weapon_chance;
                let curr_soft_pity_increment = *COMMON_SOFT_PITY_INCREMENT;
                let curr_character_soft_pity = *COMMON_CHARACTER_SOFT_PITY;
                let curr_character_pity = *COMMON_CHARACTER_PITY;
                let curr_weapon_soft_pity = *COMMON_WEAPON_SOFT_PITY;
                let curr_weapon_pity_value = *COMMON_WEAPON_PITY;
                let curr_limited_weapon_chance = game_data.limited_weapon_chance;
                let curr_limited_character_chance = game_data.limited_character_chance;

                if limited_successes >= 0 && five_char_successes < 7 {
                    let curr_five_star_chance_with_pity = curr_five_star_chance + curr_soft_pity_increment * std::cmp::max(curr_char_pity - curr_character_soft_pity, 0) as f64;

                    if random_value <= curr_five_star_chance_with_pity || curr_char_pity + 1 == curr_character_pity {
                        if curr_character_guaranteed || thread_rng().gen::<f64>() <= curr_limited_character_chance {
                            five_char_successes += 1;
                            curr_character_guaranteed = false;
                            curr_char_pity = 0;
                            limited_successes += 1;
                        } else {
                            curr_char_pity = 0;
                            curr_character_guaranteed = true;
                        }
                        if pull_for_character { // pull until 1 5 star
                            break;
                        }
                    } else {
                        curr_char_pity += 1;
                    }
                } else {
                    let curr_five_star_chance_with_pity = curr_five_star_weapon_chance + curr_soft_pity_increment * std::cmp::max(curr_weapon_pity - curr_weapon_soft_pity, 0) as f64;

                    if random_value <= curr_five_star_chance_with_pity || curr_weapon_pity + 1 == curr_weapon_pity_value {
                        if curr_weapon_guaranteed || thread_rng().gen::<f64>() < curr_limited_weapon_chance {
                            weapon_successes += 1;
                            curr_weapon_guaranteed = false;
                            curr_weapon_pity = 0;
                        } else {
                            curr_weapon_pity = 0;
                            curr_weapon_guaranteed = true;
                        }
                        if !pull_for_character { // pull until 1 weapon
                            break
                        }
                    } else if random_value <= curr_four_star_chance {
                        four_char_success += 1;
                    } else {
                        three_char_success += 1;
                    }
                    curr_char_pity += 1;
                }
            }

            results.push((pulls, limited_successes, weapon_successes, four_char_success, three_char_success));
        }
        results
    }
}