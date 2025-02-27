mod Wuwa;
mod Arknights;
mod Honkai;

use crate::Honkai::Honkai::{h_simulate_game, GameData};
use crate::Arknights::Arknights::a_simulate_game;
use crate::Wuwa::Wuwa::w_simulate_game;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::sync::Arc;
use std::sync::mpsc;
use std::thread;

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

/// Simulates Honkai games and writes results to CSV files.
fn simulate_honkai_games(num_threads: usize, num_simulations_per_thread: usize) -> io::Result<()> {
    let games = vec![
        ("hsr", GameData::new(0.008, 0.5, 0.75)),
        ("genshin", GameData::new(0.007, 0.55, 0.75)),
        ("zzz", GameData::new(0.01, 0.5, 0.75)),
    ];

    let games = Arc::new(games);

    for game in games.iter() {
        let game_name = game.0.to_string();
        let game_data = Arc::new(game.1.clone());

        // Simulate character banner
        let char_results = simulate_in_threads(
            num_threads,
            num_simulations_per_thread,
            Arc::clone(&game_data),
            true,
        )?;
        write_to_csv(
            &format!("data/{}/character.csv", game_name),
            "Pulls,Limited,Weapons,Characters,Three Stars\n",
            char_results,
        )?;

        // Simulate weapon banner
        let weapon_results = simulate_in_threads(
            num_threads,
            num_simulations_per_thread,
            Arc::clone(&game_data),
            false,
        )?;
        write_to_csv(
            &format!("data/{}/weapon.csv", game_name),
            "Pulls,Limited,Weapons,Characters,Three Stars\n",
            weapon_results,
        )?;
    }

    Ok(())
}

/// Simulates games in multiple threads and collects results.
fn simulate_in_threads(
    num_threads: usize,
    num_simulations_per_thread: usize,
    game_data: Arc<GameData>,
    is_character_banner: bool,
) -> io::Result<Vec<Vec<i32>>> {
    let (tx, rx) = mpsc::channel();

    let mut handles = vec![];
    for _ in 0..num_threads {
        let tx = tx.clone();
        let game_data = Arc::clone(&game_data);

        let handle = thread::spawn(move || {
            let results = h_simulate_game(&game_data, num_simulations_per_thread as i32, is_character_banner);
            if let Err(e) = tx.send(results) {
                eprintln!("Failed to send results: {}", e);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("Thread panicked: {:?}", e);
        }
    }

    drop(tx);

    let mut all_results = Vec::new();
    for received_results in rx {
        all_results.extend(received_results);
    }

    Ok(all_results)
}

/// Simulates Wuwa and Arknights games and writes results to CSV files.
fn simulate_other_games(num_threads: usize, num_simulations_per_thread: usize) -> io::Result<()> {
    // Wuwa simulation
    let wuwa_results = simulate_game_with_threads(num_threads, num_simulations_per_thread, w_simulate_game)?;
    write_to_csv(
        "data/wuwa/data.csv",
        "Pulls,Five Stars,Four Stars,Limited Four Stars,Three Stars\n",
        wuwa_results,
    )?;

    // Arknights simulation
    let arknights_results = simulate_game_with_threads(num_threads, num_simulations_per_thread, a_simulate_game)?;
    write_to_csv(
        "data/arknights/data.csv",
        "Pulls,Six Stars,Five Stars,Four Stars,Three Stars\n",
        arknights_results,
    )?;

    Ok(())
}

/// Simulates a game using multiple threads and collects results.
fn simulate_game_with_threads<F>(
    num_threads: usize,
    num_simulations_per_thread: usize,
    simulate_fn: F,
) -> io::Result<Vec<Vec<i32>>>
where
    F: Fn(i32) -> Vec<Vec<i32>> + Send + Sync + 'static,
{
    let (tx, rx) = mpsc::channel();

    let simulate_fn = Arc::new(simulate_fn);
    let mut handles = vec![];

    for _ in 0..num_threads {
        let tx = tx.clone();
        let simulate_fn = Arc::clone(&simulate_fn);

        let handle = thread::spawn(move || {
            let results = simulate_fn(num_simulations_per_thread as i32);
            if let Err(e) = tx.send(results) {
                eprintln!("Failed to send results: {}", e);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("Thread panicked: {:?}", e);
        }
    }

    drop(tx);

    let mut all_results = Vec::new();
    for received_results in rx {
        all_results.extend(received_results);
    }

    Ok(all_results)
}

fn main() -> io::Result<()> {
    let num_simulations = 1_000_000;
    let num_threads = std::thread::available_parallelism()?.get();
    let num_simulations_per_thread = num_simulations / num_threads;

    // Simulate Honkai games
    simulate_honkai_games(num_threads, num_simulations_per_thread)?;

    // Simulate Wuwa and Arknights games
    simulate_other_games(num_threads, num_simulations_per_thread)?;

    Ok(())
}
