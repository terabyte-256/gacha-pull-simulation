mod wuwa;
mod arknights;
mod hoyo;

use crate::hoyo::hoyo::{h_simulate_game, GameData};
use crate::arknights::arknights::a_simulate_game;
use crate::wuwa::wuwa::w_simulate_game;
use std::fs::{self, File};
use std::io::{self, Write, BufWriter};
use std::path::Path;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use clap::{Command, Arg};
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};

/// Creates a CSV writer with header
fn create_csv_writer(filepath: &str, header: &str) -> io::Result<BufWriter<File>> {
    // Ensure directory exists
    let path = Path::new(filepath);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(filepath)?;
    let mut writer = BufWriter::new(file);

    // Write header
    writer.write_all(header.as_bytes())?;
    writer.write_all(b"\n")?;

    Ok(writer)
}

/// Writes a chunk of data to CSV
fn write_chunk_to_csv(writer: &mut BufWriter<File>, chunk: &[(i32, i32, i32, i32, i32)]) -> io::Result<()> {
    for (p1, p2, p3, p4, p5) in chunk {
        writer.write_all(format!("{},{},{},{},{}\n", p1, p2, p3, p4, p5).as_bytes())?;
    }
    Ok(())
}

/// Simulates HoYoverse games with streaming CSV output
fn simulate_hoyo_games(num_simulations: u64, chunk_size: u64) -> io::Result<()> {
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
        let pb_char = multi_progress.add(ProgressBar::new(num_simulations));
        pb_char.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg} ({eta})")
            .unwrap());
        pb_char.set_message(format!("{} character pulls", game_name));

        // Create CSV writer for character pulls
        let char_filepath = format!("data/{}/character.csv", game_name);
        let char_writer = Arc::new(Mutex::new(create_csv_writer(
            &char_filepath,
            "Pulls,Limited,Weapon,FourStar,ThreeStar"
        )?));

        // Generate chunks of simulations
        let chunks = (0..num_simulations).step_by(chunk_size as usize)
            .map(|start| std::cmp::min(chunk_size, num_simulations - start))
            .collect::<Vec<_>>();

        // Process chunks in parallel
        chunks.par_iter().for_each(|&chunk_size| {
            let results = h_simulate_game(game_data, chunk_size as i32, true);

            // Write chunk to CSV
            let mut writer = char_writer.lock().unwrap();
            if let Err(e) = write_chunk_to_csv(&mut writer, &results) {
                eprintln!("Error writing to CSV: {}", e);
            }

            // Update progress bar
            pb_char.inc(chunk_size);
        });

        // Flush the writer
        let mut writer = char_writer.lock().unwrap();
        writer.flush()?;
        drop(writer);

        pb_char.finish_with_message(format!("{} character pulls completed", game_name));

        // Weapon banner simulation
        let pb_weapon = multi_progress.add(ProgressBar::new(num_simulations));
        pb_weapon.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg} ({eta})")
            .unwrap());
        pb_weapon.set_message(format!("{} weapon pulls", game_name));

        // Create CSV writer for weapon pulls
        let weapon_filepath = format!("data/{}/weapon.csv", game_name);
        let weapon_writer = Arc::new(Mutex::new(create_csv_writer(
            &weapon_filepath,
            "Pulls,Limited,Weapon,FourStar,ThreeStar"
        )?));

        // Process chunks in parallel
        chunks.par_iter().for_each(|&chunk_size| {
            let results = h_simulate_game(game_data, chunk_size as i32, false);

            // Write chunk to CSV
            let mut writer = weapon_writer.lock().unwrap();
            if let Err(e) = write_chunk_to_csv(&mut writer, &results) {
                eprintln!("Error writing to CSV: {}", e);
            }

            // Update progress bar
            pb_weapon.inc(chunk_size);
        });

        // Flush the writer
        let mut writer = weapon_writer.lock().unwrap();
        writer.flush()?;
        drop(writer);

        pb_weapon.finish_with_message(format!("{} weapon pulls completed", game_name));
    }

    Ok(())
}

/// Simulates other games with streaming CSV output
fn simulate_other_games(num_simulations: u64, chunk_size: u64) -> io::Result<()> {
    let multi_progress = MultiProgress::new();

    // Ensure directories exist
    fs::create_dir_all("data/wuwa")?;
    fs::create_dir_all("data/arknights")?;

    // Generate chunks of simulations
    let chunks = (0..num_simulations).step_by(chunk_size as usize)
        .map(|start| std::cmp::min(chunk_size, num_simulations - start))
        .collect::<Vec<_>>();

    // Wuwa simulation
    let pb_wuwa = multi_progress.add(ProgressBar::new(num_simulations));
    pb_wuwa.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg} ({eta})")
        .unwrap());
    pb_wuwa.set_message("Wuwa pulls");

    // Create CSV writer for Wuwa
    let wuwa_writer = Arc::new(Mutex::new(create_csv_writer(
        "data/wuwa/wuwa.csv",
        "Pulls,FiveStar,FourStar,LimitedFourStar,ThreeStar"
    )?));

    // Process chunks in parallel
    chunks.par_iter().for_each(|&chunk_size| {
        let results = w_simulate_game(chunk_size as i32);

        // Write chunk to CSV
        let mut writer = wuwa_writer.lock().unwrap();
        if let Err(e) = write_chunk_to_csv(&mut writer, &results) {
            eprintln!("Error writing to CSV: {}", e);
        }

        // Update progress bar
        pb_wuwa.inc(chunk_size);
    });

    // Flush the writer
    let mut writer = wuwa_writer.lock().unwrap();
    writer.flush()?;
    drop(writer);

    pb_wuwa.finish_with_message("Wuwa pulls completed");

    // Arknights simulation
    let pb_arknights = multi_progress.add(ProgressBar::new(num_simulations));
    pb_arknights.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg} ({eta})")
        .unwrap());
    pb_arknights.set_message("Arknights pulls");

    // Create CSV writer for Arknights
    let arknights_writer = Arc::new(Mutex::new(create_csv_writer(
        "data/arknights/arknights.csv",
        "Pulls,SixStar,FiveStar,FourStar,ThreeStar"
    )?));

    // Process chunks in parallel
    chunks.par_iter().for_each(|&chunk_size| {
        let results = a_simulate_game(chunk_size as i32);

        // Write chunk to CSV
        let mut writer = arknights_writer.lock().unwrap();
        if let Err(e) = write_chunk_to_csv(&mut writer, &results) {
            eprintln!("Error writing to CSV: {}", e);
        }

        // Update progress bar
        pb_arknights.inc(chunk_size);
    });

    // Flush the writer
    let mut writer = arknights_writer.lock().unwrap();
    writer.flush()?;
    drop(writer);

    pb_arknights.finish_with_message("Arknights pulls completed");

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
            .arg(
                Arg::new("chunk-size")
                    .short('c')
                    .long("chunk-size")
                    .help("Size of chunks for processing")
                    .default_value("10000")
                    .value_parser(clap::value_parser!(u64))
            )
            .get_matches();

    let num_simulations = matches.get_one::<u64>("simulations").cloned().unwrap_or(1000000);
    let chunk_size = matches.get_one::<u64>("chunk-size").cloned().unwrap_or(10000);

    println!("Starting simulations with {} total pulls for each game type", num_simulations);
    println!("Using chunk size of {} for memory efficiency", chunk_size);

    // Simulate Honkai games
    simulate_hoyo_games(num_simulations, chunk_size)?;

    // Simulate Wuwa and Arknights games
    simulate_other_games(num_simulations, chunk_size)?;

    println!("All simulations completed successfully!");

    Ok(())
}
