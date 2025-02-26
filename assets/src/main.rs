use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use Honkai;
use Wuwa;
use Honkai;


fn honkai_write_to_csv(filename: &str, data: Vec<(i32, i32, i32, i32)>) {
    let file = File::create(filename).unwrap();
    let mut writer = std::io::BufWriter::new(file);

    writer.write_all("Pulls,Limited,Weapons,Characters\n".as_bytes()).unwrap();

    for (pulls, limited, weapons, characters) in data {
        writer.write_all(format!("{},{},{},{}\n", pulls, limited, weapons, characters).as_bytes()).unwrap();
    }
}


fn simulate_honkai_games(num_threads: i32, num_simulations_per_thread: i32) {
    let games = [
        ("hsr", GameData::new(0.008, 0.5, 0.75)),
        ("genshin", GameData::new(0.007, 0.55, 0.75)),
        ("zzz", GameData::new(0.01, 0.5, 0.75)),
    ];

    for (game_name, game_data) in games.iter() {
        // Character banner simulation
        let char_results: Vec<(i32, i32, i32, i32)> = (0..num_threads)
            .map(|_| simulate_game(game_data, num_simulations_per_thread, true))
            .flatten()
            .collect();
        honkai_write_to_csv(&format!("{}_character.csv", game_name), char_results);

        // Weapon banner simulation
        let weapon_results: Vec<(i32, i32, i32, i32)> = (0..num_threads)
            .map(|_| simulate_game(game_data, num_simulations_per_thread, false))
            .flatten()
            .collect();
        honkai_write_to_csv(&format!("{}_weapon.csv", game_name), weapon_results);
    }
}

fn main() {
    let num_simulations = 1000000;
    let num_threads = 4;
    let num_simulations_per_thread = num_simulations / num_threads;

    simulate_honkai_games(num_threads, num_simulations_per_thread);

    let wuwa_results: Vec<(i32, i32, i32, i32, i32)> = (0..num_threads)
        .map(|_| Wuwa::simulate_game(num_simulations_per_thread))
        .flatten()
        .collect();

    let arknights_results: Vec<(i32, i32, i32, i32)> = (0..num_threads)
        .map(|_| Arknights::pull(num_simulations_per_thread))
        .flatten()
        .collect();

    let wuwa_file = File::create("wuwa_data.csv").unwrap();
    let mut wuwa_writer = std::io::BufWriter::new(wuwa_file);

    writer.write_all("Pulls,Five Stars,Four Stars,Limited Four Stars,Three Stars\n".as_bytes()).unwrap();

    for (pulls, five_star, four_star, limited_four_star, three_star) in wuwa_results {
        writer.write_all(format!("{},{},{},{},{}\n", pulls, five_star, four_star, limited_four_star, three_star).as_bytes()).unwrap();
    }

    let arknights_file = File::create("arknights_data.csv").unwrap();
    let mut arknights_writer = std::io::BufWriter::new(arknights_file);

    arknights_writer.write_all("Pulls,Six Stars,Five Stars,Four Stars,Three Stars\n".as_bytes()).unwrap();

    for (pulls, six_star, five_star, four_star, three_star) in arknights_results {
        arknights_writer.write_all(format!("{},{},{},{},{},{}\n", pulls, six_star, five_star, four_star, three_star).as_bytes()).unwrap();
    }

}
