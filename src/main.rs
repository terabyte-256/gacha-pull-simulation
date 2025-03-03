mod Wuwa;
mod Arknights;
mod Honkai;

use crate::Honkai::honkai::{h_simulate_game, GameData};
use crate::Arknights::arknights::a_simulate_game;
use crate::Wuwa::wuwa::w_simulate_game;
use std::fs::{self, File};
use std::io::{self, Write};
use rayon::prelude::*;
use clap::{App, Arg};

/// Writes simulation results to a CSV file.
fn write_to_csv(filepath: &str, header: &str, data: Vec<Vec<i32>>) -> io::Result<()> {
    // Ensure directory exists
    let path = Path::new(filepath);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(filepath)?;
    let mut writer = io::BufWriter::new(file);

    // Write header
    writer.write_all(header.as_bytes())?;

    // Write data rows
    for row in data {
        let row_str = row
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(",");
        writer.write_all(format!("{}\n", row_str).as_bytes())?;
    }

    Ok(())
}

fn simulate_hoyo_games(num_simulations: usize) {
let games = [
        ("hsr", GameData::new(0.008, 0.5, 0.75)),
        ("genshin", GameData::new(0.007, 0.55, 0.75)),
        ("zzz", GameData::new(0.01, 0.5, 0.75)),
    ];

    for (game_name, game_data) in games.iter() {
        // Character banner simulation
        let char_results: Vec<(i32, i32, i32, i32, i32)>  = (0..num_threads)
            .map(|_| simulate_game(game_data, num_simulations_per_thread, true))
            .flatten()
            .collect();
        write_to_csv(&format!("data/{}/character.csv", game_name), char_results);

        // Weapon banner simulation
        let weapon_results: Vec<(i32, i32, i32, i32, i32)>  = (0..num_threads)
            .map(|_| simulate_game(game_data, num_simulations_per_thread, false))
            .flatten()
            .collect();
        write_to_csv(&format!("data/{}/weapon.csv", game_name), weapon_results);
}

fn simulate_other_games(num_simulations: usize) {
    let wuwa_results: Vec<(i32, i32, i32, i32, i32)> = (0..num_threads)
        .map(|_| Wuwa::simulate_game(num_simulations_per_thread, true))
        .flatten()
        .collect();

    let arknights_results: Vec<(i32, i32, i32, i32, i32)> = (0..num_threads)
        .map(|_| Arknights::pull(num_simulations_per_thread))
        .flatten()
        .collect();

    // Ensure directories exist
    fs::create_dir_all("data/wuwa").unwrap();
    fs::create_dir_all("data/arknights").unwrap();

    //Write data to CSV files
    write_to_csv(&format!("data/wuwa/wuwa.csv"), wuwa_results);
    write_to_csv(&format!("data/arknights/arknights.csv"), arknights_results);
}

fn main() -> io::Result<()> {
    let matches = App::new("Gacha Simulator")
        .arg(Arg::with_name("simulations")
                .short("n")
                .default_value("1000000")
                .help("Number of simulations to run"))
        .get_matches();

    let num_simulations = matches.value_of("simulations")
        .unwrap()
        .parse()
        .unwrap_or(1000000);

    // Simulate Honkai games
    simulate_honkai_games(num_simulations);

    // Simulate Wuwa and arknights games
    simulate_other_games(num_simulations);

    Ok(())
}
