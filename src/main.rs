mod Honkai;
mod Wuwa;
mod Arknights;

use crate::Honkai::Honkai::{h_simulate_game, GameData};
use std::fs::{self, File};
use std::io::{Write};
use std::path::Path;
use std::thread;
use std::sync::mpsc;
use crate::Arknights::Arknights::a_simulate_game;
use crate::Wuwa::Wuwa::w_simulate_game;

fn honkai_write_to_csv(filepath: &str, data: Vec<(i32, i32, i32, i32, i32)>) {
    // Ensure directory exists
    let path = Path::new(filepath);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    let file = File::create(filepath).unwrap();
    let mut writer = std::io::BufWriter::new(file);

    writer.write_all("Pulls,Limited,Weapons,Characters,Three Stars\n".as_bytes()).unwrap();

    for (pulls, limited, weapons, characters, three_stars) in data {
        writer.write_all(format!("{},{},{},{},{}\n", pulls, limited, weapons, characters, three_stars).as_bytes()).unwrap();
    }
}

fn simulate_honkai_games(num_threads: i32, num_simulations_per_thread: i32) {
    let games = [
        ("hsr", GameData::new(0.008, 0.5, 0.75)),
        ("genshin", GameData::new(0.007, 0.55, 0.75)),
        ("zzz", GameData::new(0.01, 0.5, 0.75)),
    ];

    for (game_name, game_data) in games.iter() {
        let game_name = game_name.to_string(); // Create owned copy of game name

        // Character banner simulation
        let (tx, rx) = mpsc::channel();

        for _ in 0..num_threads {
            let tx = tx.clone();
            let game_data = game_data.clone();
            let game_name_copy = game_name.clone();

            thread::spawn(move || {
                let results = h_simulate_game(&game_data, num_simulations_per_thread, true);
                tx.send((game_name_copy, results)).unwrap();
            });
        }

        drop(tx);

        let mut char_results = Vec::new();
        for (received_game_name, received_results) in rx {
            char_results.extend(received_results);
        }

        honkai_write_to_csv(&format!("data/{}/character.csv", game_name), char_results);

        // Weapon banner simulation
        let (tx, rx) = mpsc::channel();

        for _ in 0..num_threads {
            let tx = tx.clone();
            let game_data = game_data.clone();
            let game_name_copy = game_name.clone();

            thread::spawn(move || {
                let results = h_simulate_game(&game_data, num_simulations_per_thread, false);
                tx.send((game_name_copy, results)).unwrap();
            });
        }

        drop(tx);

        let mut weapon_results = Vec::new();
        for (received_game_name, received_results) in rx {
            weapon_results.extend(received_results);
        }

        honkai_write_to_csv(&format!("data/{}/weapon.csv", game_name), weapon_results);
    }
}

fn main() {
    let num_simulations = 1000000;
    let num_threads = 4;
    let num_simulations_per_thread = num_simulations / num_threads;

    simulate_honkai_games(num_threads, num_simulations_per_thread);

    // Wuwa simulation with actual threads
    let (wuwa_tx, wuwa_rx) = mpsc::channel();

    for _ in 0..num_threads {
        let tx = wuwa_tx.clone();
        thread::spawn(move || {
            let results = w_simulate_game(num_simulations_per_thread);
            tx.send(results).unwrap();
        });
    }

    drop(wuwa_tx);

    let mut wuwa_results = Vec::new();
    for received_results in wuwa_rx {
        wuwa_results.extend(received_results);
    }

    // Arknights simulation with actual threads
    let (arknights_tx, arknights_rx) = mpsc::channel();

    for _ in 0..num_threads {
        let tx = arknights_tx.clone();
        thread::spawn(move || {
            let results = a_simulate_game(num_simulations_per_thread);
            tx.send(results).unwrap();
        });
    }

    drop(arknights_tx);

    let mut arknights_results = Vec::new();
    for received_results in arknights_rx {
        arknights_results.extend(received_results);
    }

    // Ensure directories exist
    fs::create_dir_all("data/wuwa").unwrap();
    fs::create_dir_all("data/arknights").unwrap();

    let wuwa_file = File::create("data/wuwa/data.csv").unwrap();
    let mut wuwa_writer = std::io::BufWriter::new(wuwa_file);

    wuwa_writer.write_all("Pulls,Five Stars,Four Stars,Limited Four Stars,Three Stars\n".as_bytes()).unwrap();

    for (pulls, five_star, four_star, limited_four_star, three_star) in wuwa_results {
        wuwa_writer.write_all(format!("{},{},{},{},{}\n", pulls, five_star, four_star, limited_four_star, three_star).as_bytes()).unwrap();
    }

    let arknights_file = File::create("data/arknights/data.csv").unwrap();
    let mut arknights_writer = std::io::BufWriter::new(arknights_file);

    arknights_writer.write_all("Pulls,Six Stars,Five Stars,Four Stars,Three Stars\n".as_bytes()).unwrap();

    for (pulls, six_star, five_star, four_star, three_star) in arknights_results {
        arknights_writer.write_all(format!("{},{},{},{},{}\n", pulls, six_star, five_star, four_star, three_star).as_bytes()).unwrap();
    }
}
