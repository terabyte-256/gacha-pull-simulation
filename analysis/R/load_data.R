#' Data Loading Functions for Gacha Pull Simulation
#'
#' This file contains functions to load and prepare data from simulation results.
#' The main functions are:
#' - load_game_data: Loads CSV data for all games from the data directory
#' - prepare_combined_data: Combines data from all games for comparative analysis
#'
#' @author terabyte-256
#' @date 2025-03-10

# Required packages
library(dplyr)
library(data.table)  # For fread function

#' Load game data from CSV files in the data directory
#' @return A list containing data frames for each game
load_game_data <- function() {
    # Find the project root directory by looking for the data directory
    possible_paths <- c(
        "data",             # If running from project root
        "../data",          # If running from analysis/
        "../../data",       # If running from analysis/R/
        "./data",           # Alternative
        "../../../data"     # Deeper directory
    )

    # Find the first path that exists
    data_path <- NULL
    for (path in possible_paths) {
        if (dir.exists(path)) {
            data_path <- path
            cat("Found data directory at:", normalizePath(data_path), "\n")
            break
        }
    }

    # If no valid path is found
    if (is.null(data_path)) {
        stop("Could not locate the data directory. Make sure you're running from the project directory or a subdirectory.")
    }

    # List files to confirm path is correct
    cat("Data directory contents:\n")
    print(list.files(data_path, recursive = TRUE))

    # Read the simulation results from the CSV files with try-catch for error handling
    tryCatch({
        # Using fread instead of read.csv for better performance
        results_hsr_char <- fread(file.path(data_path, "hsr/character.csv"))
        results_hsr_weapon <- fread(file.path(data_path, "hsr/weapon.csv"))
        results_genshin_char <- fread(file.path(data_path, "genshin/character.csv"))
        results_genshin_weapon <- fread(file.path(data_path, "genshin/weapon.csv"))
        results_zzz_char <- fread(file.path(data_path, "zzz/character.csv"))
        results_zzz_weapon <- fread(file.path(data_path, "zzz/weapon.csv"))
        results_arknights <- fread(file.path(data_path, "arknights/data.csv"))
        results_wuwa <- fread(file.path(data_path, "wuwa/data.csv"))
    }, error = function(e) {
        cat("Error reading data files:", e$message, "\n")
        stop("Failed to read data files. Please check the paths and file existence.")
    })

    # Convert data.tables to data.frames for consistent behavior with dplyr
    results_hsr_char <- as.data.frame(results_hsr_char)
    results_hsr_weapon <- as.data.frame(results_hsr_weapon)
    results_genshin_char <- as.data.frame(results_genshin_char)
    results_genshin_weapon <- as.data.frame(results_genshin_weapon)
    results_zzz_char <- as.data.frame(results_zzz_char)
    results_zzz_weapon <- as.data.frame(results_zzz_weapon)
    results_arknights <- as.data.frame(results_arknights)
    results_wuwa <- as.data.frame(results_wuwa)

    # Standardize column names for Arknights
    colnames(results_arknights) <- c("Pulls", "Six_Stars", "Five_Stars", "Four_Stars", "Three_Stars")

    # Standardize column names for Wuwa
    colnames(results_wuwa) <- c("Pulls", "Five_Stars", "Four_Stars", "Limited_Four_Stars", "Three_Stars")

    # Combine character and weapon data for HoYo games with banner type
    results_hsr <- bind_rows(
        mutate(results_hsr_char, Banner = "Character"),
        mutate(results_hsr_weapon, Banner = "Weapon")
    )

    results_genshin <- bind_rows(
        mutate(results_genshin_char, Banner = "Character"),
        mutate(results_genshin_weapon, Banner = "Weapon")
    )

    results_zzz <- bind_rows(
        mutate(results_zzz_char, Banner = "Character"),
        mutate(results_zzz_weapon, Banner = "Weapon")
    )

    # Add game labels
    results_hsr$Game <- "HSR"
    results_genshin$Game <- "Genshin"
    results_zzz$Game <- "ZZZ"
    results_arknights$Game <- "Arknights"
    results_wuwa$Game <- "Wuwa"

    # Return as a list
    list(
        hsr = results_hsr,
        genshin = results_genshin,
        zzz = results_zzz,
        arknights = results_arknights,
        wuwa = results_wuwa
    )
}

#' Prepare combined data for all games with standardized columns
#' @param data_list List of data frames returned by load_game_data()
#' @return A combined data frame with data from all games
prepare_combined_data <- function(data_list) {
    # For HoYo games (HSR, Genshin, ZZZ)
    hoyo_games <- bind_rows(
        data_list$hsr,
        data_list$genshin,
        data_list$zzz
    )

    # For Arknights, create comparable columns
    arknights <- data_list$arknights %>%
    mutate(
        Banner = "Overall",
        Limited = Six_Stars,  # Using Six_Stars as equivalent to Limited
        Weapons = 0  # No weapons in Arknights data
    )

    # For Wuwa, create comparable columns
    wuwa <- data_list$wuwa %>%
    mutate(
        Banner = "Overall",
        Limited = Five_Stars,  # Using Five_Stars as equivalent to Limited
        Limited_Four = Limited_Four_Stars,
        Weapons = 0  # No specific weapon data in Wuwa
    )

    # Combine all with common columns
    all_games <- bind_rows(
        hoyo_games,  # Fixed from honkai_games to hoyo_games
        arknights,
        wuwa
    )

    # Convert Game column to factor with specified levels
    all_games$Game <- factor(
        all_games$Game,
        levels = c("HSR", "Genshin", "ZZZ", "Arknights", "Wuwa")
    )

    all_games
}
