use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};
use std::thread;
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
}

struct GameData {
    five_star_weapon_chance: f64,
    limited_character_chance: f64,
    limited_weapon_chance: f64,
}

impl GameData {
    fn new(five_star_weapon_chance: f64, limited_character_chance: f64, limited_weapon_chance: f64) -> Self {
        GameData {
            five_star_weapon_chance,
            limited_character_chance,
            limited_weapon_chance,
        }
    }
}

fn simulate_game(game_data: &GameData, num_simulations: i32) -> Vec<(i32, i32, i32, i32)> {
    let mut results = Vec::new();
    for _ in 0..num_simulations {
        let mut pulls = 0;
        let mut char_successes = 0;
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
            let curr_five_star_weapon_chance = game_data.five_star_weapon_chance;
            let curr_soft_pity_increment = *COMMON_SOFT_PITY_INCREMENT;
            let curr_character_soft_pity = *COMMON_CHARACTER_SOFT_PITY;
            let curr_character_pity = *COMMON_CHARACTER_PITY;
            let curr_weapon_soft_pity = *COMMON_WEAPON_SOFT_PITY;
            let curr_weapon_pity_value = *COMMON_WEAPON_PITY;
            let curr_limited_weapon_chance = game_data.limited_weapon_chance;
            let curr_limited_character_chance = game_data.limited_character_chance;

            if char_successes < 7 {
                let curr_five_star_chance_with_pity = curr_five_star_chance + curr_soft_pity_increment * std::cmp::max(curr_char_pity - curr_character_soft_pity, 0) as f64;

                if random_value <= curr_five_star_chance_with_pity || curr_char_pity + 1 == curr_character_pity {
                    if curr_character_guaranteed || thread_rng().gen::<f64>() <= curr_limited_character_chance {
                        char_successes += 1;
                        curr_character_guaranteed = false;
                        curr_char_pity = 0;
                        limited_successes += 1;
                    } else {
                        curr_char_pity = 0;
                        curr_character_guaranteed = true;
                    }
                    break;
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
                } else {
                    curr_weapon_pity += 1;
                }
            }
        }

        results.push((pulls, limited_successes, weapon_successes, char_successes));
    }

    results
}

fn write_to_csv(filename: &str, data: Vec<(i32, i32, i32, i32)>) {
    let file = File::create(filename).unwrap();
    let mut writer = std::io::BufWriter::new(file);

    writer.write_all("Pulls,Limited,Weapons,Characters\n".as_bytes()).unwrap();

    for (pulls, limited, weapons, characters) in data {
        writer.write_all(format!("{},{},{},{}\n", pulls, limited, weapons, characters).as_bytes()).unwrap();
    }
}

fn main() {
    let num_simulations = 1000000;
    let game_data_hsr = GameData::new(0.008, 0.5, 0.75);
    let game_data_genshin = GameData::new(0.007, 0.55, 0.75);
    let game_data_zzz = GameData::new(0.01, 0.5, 0.75);

    let num_threads = 4;
    let num_simulations_per_thread = num_simulations / num_threads;

    let hsr_results: Vec<(i32, i32, i32, i32)> = (0..num_threads)
        .map(|_| {
            let results: Vec<(i32, i32, i32, i32)> = simulate_game(&game_data_hsr, num_simulations_per_thread);
            results
        })
        .flatten()
        .collect();

    let genshin_results: Vec<(i32, i32, i32, i32)> = (0..num_threads)
        .map(|_| {
            let results: Vec<(i32, i32, i32, i32)> = simulate_game(&game_data_genshin, num_simulations_per_thread);
            results
        })
        .flatten()
        .collect();

    let zzz_results: Vec<(i32, i32, i32, i32)> = (0..num_threads)
        .map(|_| {
            let results: Vec<(i32, i32, i32, i32)> = simulate_game(&game_data_zzz, num_simulations_per_thread);
            results
        })
        .flatten()
        .collect();

    write_to_csv("hsr.csv", hsr_results);
    write_to_csv("genshin.csv", genshin_results);
    write_to_csv("zzz.csv", zzz_results);
}