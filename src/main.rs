mod Wuwa;
mod Arknights;
mod Hoyo;

use crate::Hoyo::hoyo::{h_simulate_game, GameData};
use crate::Arknights::arknights::a_simulate_game;
use crate::Wuwa::wuwa::w_simulate_game;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use rayon::prelude::*;
use clap::{Command, Arg, ArgAction};
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};

/// Writes simulation results to a CSV file.
fn write_to_csv(filepath: &str, header: &str, data: Vec<(i32, i32, i32, i32, i32)>) -> io::Result<()> {
    // Ensure directory exists
    let path = Path::new(filepath);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(filepath)?;
    let mut writer = io::BufWriter::new(file);

    // Write header
    writer.write_all(header.as_bytes())?;
    writer.write_all(b"\n")?;

    // Write data rows
    for (p1, p2, p3, p4, p5) in data {
        writer.write_all(format!("{},{},{},{},{}\n", p1, p2, p3, p4, p5).as_bytes())?;
    }

    Ok(())
}

fn simulate_hoyo_games(num_simulations: u64) -> io::Result<()> {
    let num_threads = num_cpus::get() as u64;
    let num_simulations_per_thread = num_simulations / num_threads;

    let games = [
        ("hsr", GameData::new(0.008, 0.5, 0.75)),
        ("genshin", GameData::new(0.007, 0.55, 0.75)),
        ("zzz", GameData::new(0.01, 0.5, 0.75)),
    ];

    let multi_progress = MultiProgress::new();

    for (game_name, game_data) in games.iter() {
        // Create directory
        fs::create_dir_all(format!("data/{}", game_name))?;

        // Character banner simulation
        let pb_char = multi_progress.add(ProgressBar::new(num_simulations as u64));
        pb_char.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg} ({eta})")
            .unwrap());
        pb_char.set_message(format!("{} character pulls", game_name));

        let char_results: Vec<(i32, i32, i32, i32, i32)> = (0..num_threads)
            .into_par_iter()
            .flat_map(|_| {
                let results = h_simulate_game(game_data, num_simulations_per_thread as i32, true);
                pb_char.inc(num_simulations_per_thread as u64);
                results
            })
            .collect();

        pb_char.finish_with_message(format!("{} character pulls completed", game_name));

        write_to_csv(
            &format!("data/{}/character.csv", game_name),
            "Pulls,Limited,Weapon,FourStar,ThreeStar",
            char_results
        )?;

        // Weapon banner simulation
        let pb_weapon = multi_progress.add(ProgressBar::new(num_simulations as u64));
        pb_weapon.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg} ({eta})")
            .unwrap());
        pb_weapon.set_message(format!("{} weapon pulls", game_name));

        let weapon_results: Vec<(i32, i32, i32, i32, i32)> = (0..num_threads)
            .into_par_iter()
            .flat_map(|_| {
                let results = h_simulate_game(game_data, num_simulations_per_thread as i32, false);
                pb_weapon.inc(num_simulations_per_thread as u64);
                results
            })
            .collect();

        pb_weapon.finish_with_message(format!("{} weapon pulls completed", game_name));

        write_to_csv(
            &format!("data/{}/weapon.csv", game_name),
            "Pulls,Limited,Weapon,FourStar,ThreeStar",
            weapon_results
        )?;
    }

    Ok(())
}

fn simulate_other_games(num_simulations: u64) -> io::Result<()> {
    let num_threads = num_cpus::get() as u64;
    let num_simulations_per_thread = num_simulations / num_threads;
    let multi_progress = MultiProgress::new();

    // Ensure directories exist
    fs::create_dir_all("data/wuwa")?;
    fs::create_dir_all("data/arknights")?;

    // Wuwa simulation with progress bar
    let pb_wuwa = multi_progress.add(ProgressBar::new(num_simulations as u64));
    pb_wuwa.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg} ({eta})")
        .unwrap());
    pb_wuwa.set_message("Wuwa pulls");

    let wuwa_results: Vec<(i32, i32, i32, i32, i32)> = (0..num_threads)
        .into_par_iter()
        .flat_map(|_| {
            let results = w_simulate_game(num_simulations_per_thread as i32);
            pb_wuwa.inc(num_simulations_per_thread as u64);
            results
        })
        .collect();

    pb_wuwa.finish_with_message("Wuwa pulls completed");

    // Arknights simulation with progress bar
    let pb_arknights = multi_progress.add(ProgressBar::new(num_simulations as u64));
    pb_arknights.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg} ({eta})")
        .unwrap());
    pb_arknights.set_message("Arknights pulls");

    let arknights_results: Vec<(i32, i32, i32, i32, i32)> = (0..num_threads)
        .into_par_iter()
        .flat_map(|_| {
            let results = a_simulate_game(num_simulations_per_thread as i32);
            pb_arknights.inc(num_simulations_per_thread as u64);
            results
        })
        .collect();

    pb_arknights.finish_with_message("Arknights pulls completed");

    // Write data to CSV files
    write_to_csv(
        "data/wuwa/wuwa.csv",
        "Pulls,FiveStar,FourStar,LimitedFourStar,ThreeStar",
        wuwa_results
    )?;

    write_to_csv(
        "data/arknights/arknights.csv",
        "Pulls,SixStar,FiveStar,FourStar,ThreeStar",
        arknights_results
    )?;

    Ok(())
}

fn main() -> io::Result<()> {
    let matches = Command::new("Gacha Simulator")
            .arg(
                Arg::new("simulations")
                    .short('n')
                    .long("simulations")
                    .help("Number of simulations to run")
                    .default_value("1000000")
                    .value_parser(clap::value_parser!(u64))
            )
            .get_matches();

    let num_simulations = matches.get_one::<u64>("simulations").cloned().unwrap_or(1000000);

    println!("Starting simulations with {} total pulls for each game type", num_simulations);

    // Simulate Honkai games
    simulate_hoyo_games(num_simulations)?;

    // Simulate Wuwa and arknights games
    simulate_other_games(num_simulations)?;

    println!("All simulations completed successfully!");

    Ok(())
}
